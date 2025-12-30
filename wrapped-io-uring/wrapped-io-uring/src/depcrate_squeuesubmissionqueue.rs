// Generated macro for SubmissionQueue (struct)
macro_rules! Depcrate_squeueSubmissionQueue {
() => {
// Module: crate::squeue
// Provides: {"SubmissionQueue"}
// Dependencies: {}
# [doc = " An io_uring instance's submission queue. This is used to send I/O requests to the kernel."] pub struct SubmissionQueue < 'a , E : EntryMarker = Entry > { head : u32 , tail : u32 , queue : & 'a Inner < E > , }
};
}
