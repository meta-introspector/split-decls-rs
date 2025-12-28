macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_903 {
    () => {
        deps!();
        impl < 'a , St : Stream + Unpin > Iterator for Iter < 'a , St > { type Item = & 'a St ; fn next (& mut self) -> Option < Self :: Item > { let st = self . 0 . next () ? ; let next = st . get_ref () ; debug_assert ! (next . is_some ()) ; next } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
    };
}

impl_903!();