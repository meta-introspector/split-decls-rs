macro_rules! deps {
    () => {
        Stride!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        unsafe impl < 'a , A > Send for Stride < 'a , A > where A : Sync { }
    };
}

impl_106!();