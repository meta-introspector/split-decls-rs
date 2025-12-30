// Generated macro for impl_238 (impl)
macro_rules! Depcrateimpl_238 {
() => {
// Module: crate
// Provides: {"impl_238"}
// Dependencies: {}
impl < T : RefCnt + Default , S : Default + Strategy < T > > Default for ArcSwapAny < T , S > { fn default () -> Self { Self :: new (T :: default ()) } }
};
}
