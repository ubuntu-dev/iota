use std::sync::Arc;

use view::View;


pub struct ViewTree<'vt> {
    left: Option<Box<ViewTree<'vt>>>,
    right: Option<Box<ViewTree<'vt>>>,
    top: Option<Box<ViewTree<'vt>>>,
    bottom: Option<Box<ViewTree<'vt>>>,
    leaf: Option<Arc<View<'vt>>>,
}

impl<'vt> ViewTree<'vt> {
    pub fn new(v: Arc<View<'vt>>) -> ViewTree<'vt> {
        ViewTree {
            left: None,
            right: None,
            top: None,
            bottom: None,
            leaf: Some(v),
        }
    }

    fn draw(&mut self) {
        match self.leaf {
            Some(ref mut l) => return l.draw(),
            None    => {}
        }

        if self.left.is_some() {
            match self.left {
                Some(ref mut tree) => tree.draw(),
                None           => {}
            }
            match self.right {
                Some(ref mut tree) => tree.draw(),
                None           => {}
            }
            return
        }

        match self.top {
            Some(ref mut tree) => tree.draw(),
            None           => {}
        }

        match self.bottom {
            Some(ref mut tree) => tree.draw(),
            None           => {}
        }
    }
}
