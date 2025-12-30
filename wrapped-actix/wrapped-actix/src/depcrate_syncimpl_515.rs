// Generated macro for impl_515 (impl)
macro_rules! Depcrate_syncimpl_515 {
() => {
// Module: crate::sync
// Provides: {"impl_515"}
// Dependencies: {}
impl < A , M > ToEnvelope < A , M > for SyncContext < A > where A : Actor < Context = Self > + Handler < M > , M : Message + Send + 'static , M :: Result : Send , { fn pack (msg : M , tx : Option < SyncSender < M :: Result > >) -> Envelope < A > { Envelope :: with_proxy (Box :: new (SyncContextEnvelope :: new (msg , tx))) } }
};
}
