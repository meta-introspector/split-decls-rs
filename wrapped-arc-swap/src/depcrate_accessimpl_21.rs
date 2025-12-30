// Generated macro for impl_21 (impl)
macro_rules! Depcrate_accessimpl_21 {
() => {
// Module: crate::access
// Provides: {"impl_21"}
// Dependencies: {}
impl < T , S : Strategy < Rc < T > > > Access < T > for ArcSwapAny < Rc < T > , S > { type Guard = DirectDeref < Rc < T > , S > ; fn load (& self) -> Self :: Guard { DirectDeref (self . load ()) } }
};
}
