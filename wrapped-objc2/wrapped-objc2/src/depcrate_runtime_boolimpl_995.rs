// Generated macro for impl_995 (impl)
macro_rules! Depcrate_runtime_boolimpl_995 {
() => {
// Module: crate::runtime::bool
// Provides: {"impl_995"}
// Dependencies: {}
impl fmt :: Debug for Bool { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (if self . as_bool () { "YES" } else { "NO" }) } }
};
}
