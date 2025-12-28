macro_rules! deps {
    () => {
        Allocator!();
        Splice!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < I : Iterator , A : Allocator > DoubleEndedIterator for Splice < '_ , I , A > { # [inline (always)] fn next_back (& mut self) -> Option < Self :: Item > { self . drain . next_back () } }
    };
}

impl_103!();