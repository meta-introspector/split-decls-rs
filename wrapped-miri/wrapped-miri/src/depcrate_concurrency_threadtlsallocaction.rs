// Generated macro for TlsAllocAction (enum)
macro_rules! Depcrate_concurrency_threadTlsAllocAction {
() => {
// Module: crate::concurrency::thread
// Provides: {"TlsAllocAction"}
// Dependencies: {}
# [doc = " What to do with TLS allocations from terminated threads"] # [derive (Clone , Copy , Debug , PartialEq)] pub enum TlsAllocAction { # [doc = " Deallocate backing memory of thread-local statics as usual"] Deallocate , # [doc = " Skip deallocating backing memory of thread-local statics and consider all memory reachable"] # [doc = " from them as \"allowed to leak\" (like global `static`s)."] Leak , }
};
}
