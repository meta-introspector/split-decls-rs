macro_rules! deps {
    () => {
        ArcSwapAny!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T : RefCnt + Default , S : Default + Strategy < T > > Default for ArcSwapAny < T , S > { fn default () -> Self { Self :: new (T :: default ()) } }
    };
}

impl_24!()