// Generated macro for impl_119 (impl)
macro_rules! Depcrate_addressimpl_119 {
() => {
// Module: crate::address
// Provides: {"impl_119"}
// Dependencies: {}
impl < A : Actor > Hash for Addr < A > { fn hash < H : Hasher > (& self , state : & mut H) { self . tx . hash (state) } }
};
}
