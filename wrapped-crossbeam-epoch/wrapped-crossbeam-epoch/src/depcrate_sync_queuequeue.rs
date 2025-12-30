// Generated macro for Queue (struct)
macro_rules! Depcrate_sync_queueQueue {
() => {
// Module: crate::sync::queue
// Provides: {"Queue"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Queue < T > { head : CachePadded < Atomic < Node < T > > > , tail : CachePadded < Atomic < Node < T > > > , }
};
}
