// Generated macro for impl_1261 (impl)
macro_rules! Depcrate_iter_formatimpl_1261 {
() => {
// Module: crate::iter_format
// Provides: {"impl_1261"}
// Dependencies: {}
impl < T > fmt :: Debug for NoPretty < T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{:?}" , self . 0) } }
};
}
