macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        unsafe impl < T : Sync , const N : usize > Sync for SmallVec < T , N > { }
    };
}

impl_14!()