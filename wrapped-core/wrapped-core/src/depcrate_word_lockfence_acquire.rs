// Generated macro for fence_acquire (function)
macro_rules! Depcrate_word_lockfence_acquire {
() => {
// Module: crate::word_lock
// Provides: {"fence_acquire"}
// Dependencies: {}
# [inline] fn fence_acquire (a : & AtomicUsize) { if cfg ! (tsan_enabled) { let _ = a . load (Ordering :: Acquire) ; } else { fence (Ordering :: Acquire) ; } }
};
}
