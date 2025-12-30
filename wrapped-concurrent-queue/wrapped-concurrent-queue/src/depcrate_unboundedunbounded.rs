// Generated macro for Unbounded (struct)
macro_rules! Depcrate_unboundedUnbounded {
() => {
// Module: crate::unbounded
// Provides: {"Unbounded"}
// Dependencies: {}
# [doc = " An unbounded queue."] pub struct Unbounded < T > { # [doc = " The head of the queue."] head : CachePadded < Position < T > > , # [doc = " The tail of the queue."] tail : CachePadded < Position < T > > , }
};
}
