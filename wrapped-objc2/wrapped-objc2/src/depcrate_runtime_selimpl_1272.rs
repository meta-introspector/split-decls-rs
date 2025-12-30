// Generated macro for impl_1272 (impl)
macro_rules! Depcrate_runtime_selimpl_1272 {
() => {
// Module: crate::runtime::sel
// Provides: {"impl_1272"}
// Dependencies: {}
impl hash :: Hash for Sel { # [inline] fn hash < H : hash :: Hasher > (& self , state : & mut H) { if cfg ! (feature = "gnustep-1-7") { self . name () . hash (state) ; } else { self . as_ptr () . hash (state) ; } } }
};
}
