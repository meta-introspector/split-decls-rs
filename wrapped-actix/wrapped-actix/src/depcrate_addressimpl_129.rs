// Generated macro for impl_129 (impl)
macro_rules! Depcrate_addressimpl_129 {
() => {
// Module: crate::address
// Provides: {"impl_129"}
// Dependencies: {}
impl < A : Actor , M : Message + Send + 'static > From < Addr < A > > for Recipient < M > where A : Handler < M > , M :: Result : Send , A :: Context : ToEnvelope < A , M > , { fn from (addr : Addr < A >) -> Self { Recipient :: new (Box :: new (addr . tx)) } }
};
}
