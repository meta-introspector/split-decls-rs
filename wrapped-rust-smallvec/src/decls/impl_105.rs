macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        unsafe impl < T , const N : usize > Sync for IntoIter < T , N > where T : Sync { }
    };
}

impl_105!();