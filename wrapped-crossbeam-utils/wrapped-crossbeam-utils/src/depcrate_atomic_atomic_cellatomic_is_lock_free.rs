// Generated macro for atomic_is_lock_free (function)
macro_rules! Depcrate_atomic_atomic_cellatomic_is_lock_free {
() => {
// Module: crate::atomic::atomic_cell
// Provides: {"atomic_is_lock_free"}
// Dependencies: {}
# [doc = " Returns `true` if operations on `AtomicCell<T>` are lock-free."] const fn atomic_is_lock_free < T > () -> bool { atomic ! { T , _a , true , false } }
};
}
