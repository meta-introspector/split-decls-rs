// Generated macro for impl_137 (impl)
macro_rules! Depcrate_addressimpl_137 {
() => {
// Module: crate::address
// Provides: {"impl_137"}
// Dependencies: {}
impl < M > Clone for WeakRecipient < M > where M : Message + Send , M :: Result : Send , { fn clone (& self) -> Self { Self { wtx : self . wtx . boxed () , } } }
};
}
