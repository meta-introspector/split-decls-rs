// Generated macro for impl_520 (impl)
macro_rules! Depcrate_syncimpl_520 {
() => {
// Module: crate::sync
// Provides: {"impl_520"}
// Dependencies: {}
impl < M > SyncContextEnvelope < M > where M : Message + Send , M :: Result : Send , { pub fn new (msg : M , tx : Option < SyncSender < M :: Result > >) -> Self { Self { tx , msg : Some (msg) } } }
};
}
