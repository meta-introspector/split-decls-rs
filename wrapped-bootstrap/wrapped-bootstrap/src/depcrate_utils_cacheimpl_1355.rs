// Generated macro for impl_1355 (impl)
macro_rules! Depcrate_utils_cacheimpl_1355 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1355"}
// Dependencies: {}
impl < T , U : ? Sized + fmt :: Debug > fmt :: Debug for Interned < T > where Self : Deref < Target = U > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s : & U = self ; f . write_fmt (format_args ! ("{s:?}")) } }
};
}
