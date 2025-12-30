// Generated macro for impl_138 (impl)
macro_rules! Depcrate_addressimpl_138 {
() => {
// Module: crate::address
// Provides: {"impl_138"}
// Dependencies: {}
impl < M > From < Recipient < M > > for WeakRecipient < M > where M : Message + Send , M :: Result : Send , { fn from (recipient : Recipient < M >) -> Self { recipient . downgrade () } }
};
}
