macro_rules! deps {
    () => {
        Strategy!();
        ArcSwapAny!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T , S : Strategy < Arc < T > > > ArcSwapAny < Arc < T > , S > { # [doc = " A convenience constructor directly from the pointed-to value."] # [doc = ""] # [doc = " Direct equivalent for `ArcSwap::new(Arc::new(val))`."] pub fn from_pointee (val : T) -> Self where S : Default , { Self :: from (Arc :: new (val)) } }
    };
}

impl_155!();