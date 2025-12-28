macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for Iter < '_ , T > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () } }
    };
}

impl_29!();