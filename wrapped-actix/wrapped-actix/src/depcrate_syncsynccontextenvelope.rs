// Generated macro for SyncContextEnvelope (struct)
macro_rules! Depcrate_syncSyncContextEnvelope {
() => {
// Module: crate::sync
// Provides: {"SyncContextEnvelope"}
// Dependencies: {}
pub (crate) struct SyncContextEnvelope < M > where M : Message + Send , { msg : Option < M > , tx : Option < SyncSender < M :: Result > > , }
};
}
