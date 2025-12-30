// Generated macro for SeqLockWriteGuard (struct)
macro_rules! Depcrate_atomic_seq_lockSeqLockWriteGuard {
() => {
// Module: crate::atomic::seq_lock
// Provides: {"SeqLockWriteGuard"}
// Dependencies: {}
# [doc = " An RAII guard that releases the lock and increments the stamp when dropped."] pub (crate) struct SeqLockWriteGuard { # [doc = " The parent lock."] lock : & 'static SeqLock , # [doc = " The stamp before locking."] state : usize , }
};
}
