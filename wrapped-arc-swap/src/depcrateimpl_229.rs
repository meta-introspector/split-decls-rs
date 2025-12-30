// Generated macro for impl_229 (impl)
macro_rules! Depcrateimpl_229 {
() => {
// Module: crate
// Provides: {"impl_229"}
// Dependencies: {}
impl < T : Default + RefCnt , S : Strategy < T > > Default for Guard < T , S > { fn default () -> Self { Self :: from (T :: default ()) } }
};
}
