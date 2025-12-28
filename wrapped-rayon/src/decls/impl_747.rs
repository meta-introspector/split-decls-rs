macro_rules! deps {
    () => {
        PanicFuseIter!();
    };
}

macro_rules! impl_747 {
    () => {
        deps!();
        impl < 'a , I > DoubleEndedIterator for PanicFuseIter < 'a , I > where I : DoubleEndedIterator , { fn next_back (& mut self) -> Option < Self :: Item > { if self . fuse . panicked () { None } else { self . base . next_back () } } }
    };
}

impl_747!()