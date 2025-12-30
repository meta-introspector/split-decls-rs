// Generated macro for impl_25 (impl)
macro_rules! Depcrate_interruptimpl_25 {
() => {
// Module: crate::interrupt
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a , I , EFN , E > IterWithErr < 'a , I , EFN > where I : Iterator , EFN : FnOnce () -> E , { # [doc = " Create a new iterator over `inner` which checks for interruptions on each iteration and calls `make_err()` to"] # [doc = " signal an interruption happened, causing no further items to be iterated from that point on."] pub fn new (inner : I , make_err : EFN , should_interrupt : & 'a AtomicBool) -> Self { IterWithErr { inner , make_err : Some (make_err) , should_interrupt , } } }
};
}
