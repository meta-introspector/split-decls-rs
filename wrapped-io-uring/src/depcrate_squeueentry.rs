// Generated macro for Entry (struct)
macro_rules! Depcrate_squeueEntry {
() => {
// Module: crate::squeue
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A 64-byte submission queue entry (SQE), representing a request for an I/O operation."] # [doc = ""] # [doc = " These can be created via opcodes in [`opcode`](crate::opcode)."] # [repr (C)] pub struct Entry (pub (crate) sys :: io_uring_sqe) ;
};
}
