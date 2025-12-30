// Generated macro for impl_140 (impl)
macro_rules! Depcrate_addressimpl_140 {
() => {
// Module: crate::address
// Provides: {"impl_140"}
// Dependencies: {}
impl < A : Actor , M : Message + Send + 'static > From < Addr < A > > for WeakRecipient < M > where A : Handler < M > , M :: Result : Send , A :: Context : ToEnvelope < A , M > , { fn from (addr : Addr < A >) -> WeakRecipient < M > { addr . downgrade () . recipient () } }
};
}
