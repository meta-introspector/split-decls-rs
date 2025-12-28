macro_rules! deps {
    () => {
        UnalignedIter!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'a , T > Iterator for UnalignedIter < 'a , T > where T : Copy , { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . ptr != self . end { unsafe { let elt = Some (ptr :: read_unaligned (self . ptr as * const T)) ; self . ptr = self . ptr . offset (size_of :: < T > () as isize) ; elt } } else { None } } }
    };
}

impl_78!()