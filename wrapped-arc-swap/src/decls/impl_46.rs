macro_rules! deps {
    () => {
        RefCnt!();
        ArcSwapAny!();
        Cache!();
        Strategy!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < A , T , S > From < A > for Cache < A , T > where A : Deref < Target = ArcSwapAny < T , S > > , T : RefCnt , S : Strategy < T > , { fn from (arc_swap : A) -> Self { Self :: new (arc_swap) } }
    };
}

impl_46!();