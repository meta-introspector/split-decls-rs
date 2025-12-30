// Generated macro for impl_133 (impl)
macro_rules! Depcrate_addressimpl_133 {
() => {
// Module: crate::address
// Provides: {"impl_133"}
// Dependencies: {}
impl < M > Hash for Recipient < M > where M : Message + Send , M :: Result : Send , { fn hash < H : Hasher > (& self , state : & mut H) { self . tx . hash () . hash (state) } }
};
}
