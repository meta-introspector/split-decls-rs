macro_rules! deps {
    () => {
        ArcSwapAny!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T : RefCnt , S : Default + Strategy < T > > From < T > for ArcSwapAny < T , S > { fn from (val : T) -> Self { Self :: with_strategy (val , S :: default ()) } }
    };
}

impl_20!()