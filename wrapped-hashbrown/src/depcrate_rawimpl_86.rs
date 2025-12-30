// Generated macro for impl_86 (impl)
macro_rules! Depcrate_rawimpl_86 {
() => {
// Module: crate::raw
// Provides: {"impl_86"}
// Dependencies: {}
impl < T : Clone , A : Allocator + Clone > RawTableClone for RawTable < T , A > { default_fn ! { # [cfg_attr (feature = "inline-more" , inline)] unsafe fn clone_from_spec (& mut self , source : & Self) { self . clone_from_impl (source) ; } } }
};
}
