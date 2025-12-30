// Generated macro for impl_num (macro)
macro_rules! Depcrate_rt_numimpl_num {
() => {
// Module: crate::rt::num
// Provides: {"impl_num"}
// Dependencies: {}
macro_rules ! impl_num { ($ ($ t : ty) ,*) => { $ (impl Numeric for $ t { fn into_u64 (self) -> u64 { self as u64 } fn from_u64 (src : u64) -> $ t { src as $ t } }) * } ; }
};
}
