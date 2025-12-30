// Generated macro for impl_152 (impl)
macro_rules! Depcrateimpl_152 {
() => {
// Module: crate
// Provides: {"impl_152"}
// Dependencies: {}
impl Hash for RelativePath { # [inline] fn hash < H > (& self , h : & mut H) where H : Hasher , { for c in self . components () { c . hash (h) ; } } }
};
}
