macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_32!()