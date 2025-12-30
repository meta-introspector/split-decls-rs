// Generated macro for Iter (struct)
macro_rules! Depcrate_interruptIter {
() => {
// Module: crate::interrupt
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " A wrapper for an inner iterator which will check for interruptions on each iteration."] pub struct Iter < I , EFN > { # [doc = " The actual iterator to yield elements from."] inner : gix_features :: interrupt :: IterWithErr < 'static , I , EFN > , }
};
}
