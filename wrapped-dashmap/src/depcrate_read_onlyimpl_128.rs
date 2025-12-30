// Generated macro for impl_128 (impl)
macro_rules! Depcrate_read_onlyimpl_128 {
() => {
// Module: crate::read_only
// Provides: {"impl_128"}
// Dependencies: {}
impl < K , V , S > ReadOnlyView < K , V , S > { pub (crate) fn new (map : DashMap < K , V , S >) -> Self { Self { map } } # [doc = " Consumes this `ReadOnlyView`, returning the underlying `DashMap`."] pub fn into_inner (self) -> DashMap < K , V , S > { self . map } }
};
}
