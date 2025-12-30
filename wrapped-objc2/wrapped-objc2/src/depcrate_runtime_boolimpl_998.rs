// Generated macro for impl_998 (impl)
macro_rules! Depcrate_runtime_boolimpl_998 {
() => {
// Module: crate::runtime::bool
// Provides: {"impl_998"}
// Dependencies: {}
impl hash :: Hash for Bool { # [inline] fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . as_bool () . hash (state) ; } }
};
}
