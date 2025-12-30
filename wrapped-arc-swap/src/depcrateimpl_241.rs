// Generated macro for impl_241 (impl)
macro_rules! Depcrateimpl_241 {
() => {
// Module: crate
// Provides: {"impl_241"}
// Dependencies: {}
impl < T , S : Strategy < Arc < T > > > ArcSwapAny < Arc < T > , S > { # [doc = " A convenience constructor directly from the pointed-to value."] # [doc = ""] # [doc = " Direct equivalent for `ArcSwap::new(Arc::new(val))`."] pub fn from_pointee (val : T) -> Self where S : Default , { Self :: from (Arc :: new (val)) } }
};
}
