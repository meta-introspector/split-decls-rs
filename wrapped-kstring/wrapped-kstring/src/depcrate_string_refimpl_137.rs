// Generated macro for impl_137 (impl)
macro_rules! Depcrate_string_refimpl_137 {
() => {
// Module: crate::string_ref
// Provides: {"impl_137"}
// Dependencies: {}
impl std :: hash :: Hash for KStringRef < '_ > { # [inline] fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . as_str () . hash (state) ; } }
};
}
