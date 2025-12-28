macro_rules! deps {
    () => {
        ArcSwapAny!();
        RefCnt!();
        Strategy!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < T : RefCnt , S : Default + Strategy < T > > From < T > for ArcSwapAny < T , S > { fn from (val : T) -> Self { Self :: with_strategy (val , S :: default ()) } }
    };
}

impl_148!();