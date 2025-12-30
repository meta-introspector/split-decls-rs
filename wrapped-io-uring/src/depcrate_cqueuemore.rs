// Generated macro for more (function)
macro_rules! Depcrate_cqueuemore {
() => {
// Module: crate::cqueue
// Provides: {"more"}
// Dependencies: {}
# [doc = " Return whether further completion events will be submitted for"] # [doc = " this same operation."] # [doc = ""] # [doc = " This corresponds to the `IORING_CQE_F_MORE` flag, and it signals to"] # [doc = " the consumer that it should expect further CQE entries after this one,"] # [doc = " still from the same original SQE request (e.g. for multishot operations)."] pub fn more (flags : u32) -> bool { flags & sys :: IORING_CQE_F_MORE != 0 }
};
}
