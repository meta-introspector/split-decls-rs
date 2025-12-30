// Generated macro for LockState (trait)
macro_rules! Depcrate_word_lockLockState {
() => {
// Module: crate::word_lock
// Provides: {"LockState"}
// Dependencies: {}
trait LockState { fn is_locked (self) -> bool ; fn is_queue_locked (self) -> bool ; fn queue_head (self) -> * const ThreadData ; fn with_queue_head (self , thread_data : * const ThreadData) -> Self ; }
};
}
