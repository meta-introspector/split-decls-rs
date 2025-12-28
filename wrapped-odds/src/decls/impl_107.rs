macro_rules! deps {
    () => {
        Stride!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        unsafe impl < 'a , A > Sync for Stride < 'a , A > where A : Sync { }
    };
}

impl_107!();