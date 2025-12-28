macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
        Unwrap!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < T > iter :: DoubleEndedIterator for Unwrap < T > where T : DoubleEndedFallibleIterator , T :: Error : core :: fmt :: Debug , { # [inline] fn next_back (& mut self) -> Option < T :: Item > { self . 0 . next_back () . unwrap () } }
    };
}

impl_127!()