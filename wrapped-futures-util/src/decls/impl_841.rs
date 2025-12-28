macro_rules! deps {
    () => {
        IterPinMut!();
    };
}

macro_rules! impl_841 {
    () => {
        deps!();
        impl < 'a , Fut > Iterator for IterPinMut < 'a , Fut > { type Item = Pin < & 'a mut Fut > ; fn next (& mut self) -> Option < Self :: Item > { if self . task . is_null () { return None ; } unsafe { let future = (* (* self . task) . future . get ()) . as_mut () . unwrap () ; let next = (* self . task) . next_all . load (Relaxed) ; self . task = next ; self . len -= 1 ; Some (Pin :: new_unchecked (future)) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_841!()