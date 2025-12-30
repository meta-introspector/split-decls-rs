// Generated macro for impl_141 (impl)
macro_rules! Depcrate_addressimpl_141 {
() => {
// Module: crate::address
// Provides: {"impl_141"}
// Dependencies: {}
impl < A : Actor , M : Message + Send + 'static > From < WeakAddr < A > > for WeakRecipient < M > where A : Handler < M > , M :: Result : Send , A :: Context : ToEnvelope < A , M > , { fn from (addr : WeakAddr < A >) -> WeakRecipient < M > { WeakRecipient :: new (Box :: new (addr . wtx)) } }
};
}
