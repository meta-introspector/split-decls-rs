macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_847 {
    () => {
        deps!();
        impl < 'a , Fut : Unpin > Iterator for Iter < 'a , Fut > { type Item = & 'a Fut ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (Pin :: get_ref) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
    };
}

impl_847!()