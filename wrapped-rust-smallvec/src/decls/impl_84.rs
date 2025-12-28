macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        unsafe impl < T : Sync , const N : usize > Sync for SmallVec < T , N > { }
    };
}

impl_84!()