// Generated macro for impl_80 (impl)
macro_rules! Depcrate_word_lockimpl_80 {
() => {
// Module: crate::word_lock
// Provides: {"impl_80"}
// Dependencies: {}
impl LockState for usize { # [inline] fn is_locked (self) -> bool { self & LOCKED_BIT != 0 } # [inline] fn is_queue_locked (self) -> bool { self & QUEUE_LOCKED_BIT != 0 } # [inline] fn queue_head (self) -> * const ThreadData { (self & QUEUE_MASK) as * const ThreadData } # [inline] fn with_queue_head (self , thread_data : * const ThreadData) -> Self { (self & ! QUEUE_MASK) | thread_data as * const _ as usize } }
};
}
