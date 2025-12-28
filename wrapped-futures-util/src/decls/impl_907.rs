macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_907 {
    () => {
        deps!();
        impl < St : Stream + Unpin > Iterator for IntoIter < St > { type Item = St ; fn next (& mut self) -> Option < Self :: Item > { let st = self . 0 . next () ? ; let next = st . into_inner () ; debug_assert ! (next . is_some ()) ; next } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
    };
}

impl_907!();