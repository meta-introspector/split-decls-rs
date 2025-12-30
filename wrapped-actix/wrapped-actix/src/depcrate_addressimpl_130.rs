// Generated macro for impl_130 (impl)
macro_rules! Depcrate_addressimpl_130 {
() => {
// Module: crate::address
// Provides: {"impl_130"}
// Dependencies: {}
impl < M > Clone for Recipient < M > where M : Message + Send , M :: Result : Send , { fn clone (& self) -> Recipient < M > { Recipient { tx : self . tx . boxed () , } } }
};
}
