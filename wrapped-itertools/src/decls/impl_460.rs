macro_rules! deps {
    () => {
        RcIter!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl < I > DoubleEndedIterator for RcIter < I > where I : DoubleEndedIterator , { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . rciter . borrow_mut () . next_back () } }
    };
}

impl_460!()