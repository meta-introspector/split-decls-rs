// Generated macro for SyncEnvelopeProxy (struct)
macro_rules! Depcrate_address_envelopeSyncEnvelopeProxy {
() => {
// Module: crate::address::envelope
// Provides: {"SyncEnvelopeProxy"}
// Dependencies: {}
pub struct SyncEnvelopeProxy < M > where M : Message + Send , M :: Result : Send , { msg : Option < M > , tx : Option < Sender < M :: Result > > , }
};
}
