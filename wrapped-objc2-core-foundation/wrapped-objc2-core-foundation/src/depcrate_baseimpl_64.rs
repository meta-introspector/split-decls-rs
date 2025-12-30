// Generated macro for impl_64 (impl)
macro_rules! Depcrate_baseimpl_64 {
() => {
// Module: crate::base
// Provides: {"impl_64"}
// Dependencies: {}
impl hash :: Hash for CFType { # [doc (alias = "CFHash")] fn hash < H : hash :: Hasher > (& self , state : & mut H) { CFHash (Some (self)) . hash (state) ; } }
};
}
