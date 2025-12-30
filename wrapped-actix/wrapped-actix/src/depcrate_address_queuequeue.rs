// Generated macro for Queue (struct)
macro_rules! Depcrate_address_queueQueue {
() => {
// Module: crate::address::queue
// Provides: {"Queue"}
// Dependencies: {}
# [doc = " The multi-producer single-consumer structure. This is not cloneable, but it"] # [doc = " may be safely shared so long as it is guaranteed that there is only one"] # [doc = " popper at a time (many pushers are allowed)."] # [derive (Debug)] pub (super) struct Queue < T > { head : AtomicPtr < Node < T > > , tail : UnsafeCell < * mut Node < T > > , }
};
}
