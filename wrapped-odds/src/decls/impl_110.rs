macro_rules! deps {
    () => {
        StrideMut!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        unsafe impl < 'a , A > Sync for StrideMut < 'a , A > where A : Sync { }
    };
}

impl_110!()