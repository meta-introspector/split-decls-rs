// Generated macro for impl_19 (impl)
macro_rules! Depcrate_accessimpl_19 {
() => {
// Module: crate::access
// Provides: {"impl_19"}
// Dependencies: {}
impl < T , S : Strategy < Arc < T > > > Access < T > for ArcSwapAny < Arc < T > , S > { type Guard = DirectDeref < Arc < T > , S > ; fn load (& self) -> Self :: Guard { DirectDeref (self . load ()) } }
};
}
