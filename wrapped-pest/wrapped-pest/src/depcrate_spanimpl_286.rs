// Generated macro for impl_286 (impl)
macro_rules! Depcrate_spanimpl_286 {
() => {
// Module: crate::span
// Provides: {"impl_286"}
// Dependencies: {}
impl Hash for Span < '_ > { fn hash < H : Hasher > (& self , state : & mut H) { (self . input as * const str) . hash (state) ; self . start . hash (state) ; self . end . hash (state) ; } }
};
}
