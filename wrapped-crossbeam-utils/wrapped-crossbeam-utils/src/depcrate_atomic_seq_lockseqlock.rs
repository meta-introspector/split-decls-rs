// Generated macro for SeqLock (struct)
macro_rules! Depcrate_atomic_seq_lockSeqLock {
() => {
// Module: crate::atomic::seq_lock
// Provides: {"SeqLock"}
// Dependencies: {}
# [doc = " A simple stamped lock."] pub (crate) struct SeqLock { # [doc = " The current state of the lock."] # [doc = ""] # [doc = " All bits except the least significant one hold the current stamp. When locked, the state"] # [doc = " equals 1 and doesn't contain a valid stamp."] state : AtomicUsize , }
};
}
