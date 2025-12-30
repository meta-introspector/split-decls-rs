// Generated macro for IterWithErr (struct)
macro_rules! Depcrate_interruptIterWithErr {
() => {
// Module: crate::interrupt
// Provides: {"IterWithErr"}
// Dependencies: {}
# [doc = " A wrapper for an inner iterator which will check for interruptions on each iteration."] pub struct IterWithErr < 'a , I , EFN > { # [doc = " The actual iterator to yield elements from."] pub inner : I , make_err : Option < EFN > , should_interrupt : & 'a AtomicBool , }
};
}
