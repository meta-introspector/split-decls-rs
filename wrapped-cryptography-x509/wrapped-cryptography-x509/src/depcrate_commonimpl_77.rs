// Generated macro for impl_77 (impl)
macro_rules! Depcrate_commonimpl_77 {
() => {
// Module: crate::common
// Provides: {"impl_77"}
// Dependencies: {}
impl < T : std :: hash :: Hash > std :: hash :: Hash for WithTlv < '_ , T > { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . value . hash (state) } }
};
}
