// Generated macro for CompletionQueue (struct)
macro_rules! Depcrate_cqueueCompletionQueue {
() => {
// Module: crate::cqueue
// Provides: {"CompletionQueue"}
// Dependencies: {}
# [doc = " An io_uring instance's completion queue. This stores all the I/O operations that have completed."] pub struct CompletionQueue < 'a , E : EntryMarker = Entry > { head : u32 , tail : u32 , queue : & 'a Inner < E > , }
};
}
