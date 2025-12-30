// Generated macro for Position (struct)
macro_rules! Depcrate_unboundedPosition {
() => {
// Module: crate::unbounded
// Provides: {"Position"}
// Dependencies: {}
# [doc = " A position in a queue."] struct Position < T > { # [doc = " The index in the queue."] index : AtomicUsize , # [doc = " The block in the linked list."] block : AtomicPtr < Block < T > > , }
};
}
