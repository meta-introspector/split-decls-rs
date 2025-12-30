// Generated macro for Inner (struct)
macro_rules! Depcrate_contextInner {
() => {
// Module: crate::context
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " Inner representation of `Context`."] # [derive (Debug)] struct Inner { # [doc = " Selected operation."] select : AtomicUsize , # [doc = " A slot into which another thread may store a pointer to its `Packet`."] packet : AtomicPtr < () > , # [doc = " Thread handle."] thread : Thread , # [doc = " Thread id."] thread_id : ThreadId , }
};
}
