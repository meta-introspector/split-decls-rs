// Generated macro for StreamPriorityKey (struct)
macro_rules! Depcrate_streamStreamPriorityKey {
() => {
// Module: crate::stream
// Provides: {"StreamPriorityKey"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct StreamPriorityKey { pub urgency : u8 , pub incremental : bool , pub id : u64 , pub readable : RBTreeAtomicLink , pub writable : RBTreeAtomicLink , pub flushable : RBTreeAtomicLink , }
};
}
