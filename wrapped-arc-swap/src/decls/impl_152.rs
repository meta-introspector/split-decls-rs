macro_rules! deps {
    () => {
        Strategy!();
        ArcSwapAny!();
        RefCnt!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < T : RefCnt + Default , S : Default + Strategy < T > > Default for ArcSwapAny < T , S > { fn default () -> Self { Self :: new (T :: default ()) } }
    };
}

impl_152!();