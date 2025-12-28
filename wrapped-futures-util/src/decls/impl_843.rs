macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_843 {
    () => {
        deps!();
        impl < 'a , Fut : Unpin > Iterator for IterMut < 'a , Fut > { type Item = & 'a mut Fut ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (Pin :: get_mut) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
    };
}

impl_843!()