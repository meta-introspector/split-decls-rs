macro_rules! deps {
    () => {
        Strategy!();
        RefCnt!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < T : RefCnt , S : sealed :: InnerStrategy < T > > Strategy < T > for S { }
    };
}

impl_129!();