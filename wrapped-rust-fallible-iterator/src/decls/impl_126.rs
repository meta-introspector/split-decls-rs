macro_rules! deps {
    () => {
        Iterator!();
        Unwrap!();
        FallibleIterator!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < T > iter :: Iterator for Unwrap < T > where T : FallibleIterator , T :: Error : core :: fmt :: Debug , { type Item = T :: Item ; # [inline] fn next (& mut self) -> Option < T :: Item > { self . 0 . next () . unwrap () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (_ , max) = self . 0 . size_hint () ; (0 , max) } }
    };
}

impl_126!();