macro_rules! deps {
    () => {
        SliceCopyIter!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < 'a , T > DoubleEndedIterator for SliceCopyIter < 'a , T > where T : Copy , { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { if self . ptr != self . end { unsafe { self . end = self . end . offset (- 1) ; let elt = Some (* self . end) ; elt } } else { None } } }
    };
}

impl_43!();