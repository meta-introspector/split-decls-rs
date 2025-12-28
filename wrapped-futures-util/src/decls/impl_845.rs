macro_rules! deps {
    () => {
        IterPinRef!();
    };
}

macro_rules! impl_845 {
    () => {
        deps!();
        impl < 'a , Fut > Iterator for IterPinRef < 'a , Fut > { type Item = Pin < & 'a Fut > ; fn next (& mut self) -> Option < Self :: Item > { if self . task . is_null () { return None ; } unsafe { let future = (* (* self . task) . future . get ()) . as_ref () . unwrap () ; let next = (* self . task) . spin_next_all (self . pending_next_all , Relaxed) ; self . task = next ; self . len -= 1 ; Some (Pin :: new_unchecked (future)) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_845!();