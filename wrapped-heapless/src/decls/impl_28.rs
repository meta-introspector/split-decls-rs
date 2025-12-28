macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_28!();