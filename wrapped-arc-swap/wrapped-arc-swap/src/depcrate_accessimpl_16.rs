// Generated macro for impl_16 (impl)
macro_rules! Depcrate_accessimpl_16 {
() => {
// Module: crate::access
// Provides: {"impl_16"}
// Dependencies: {}
impl < T : RefCnt , S : Strategy < T > > Access < T > for ArcSwapAny < T , S > { type Guard = Guard < T , S > ; fn load (& self) -> Self :: Guard { self . load () } }
};
}
