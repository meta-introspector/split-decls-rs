// Generated macro for impl_351 (impl)
macro_rules! Depcrate_index_strimpl_351 {
() => {
// Module: crate::index_str
// Provides: {"impl_351"}
// Dependencies: {}
impl < 'a > fmt :: Debug for IndexStr < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "IndexStr {{ idx: {}, string: \"{}\" }}" , self . idx , String :: from_utf8_lossy (self . as_ref ())) } }
};
}
