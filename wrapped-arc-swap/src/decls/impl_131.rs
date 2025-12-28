macro_rules! deps {
    () => {
        CaS!();
        RefCnt!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < T : RefCnt , S : sealed :: CaS < T > > CaS < T > for S { }
    };
}

impl_131!();