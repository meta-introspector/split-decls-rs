// Generated macro for prevent_overflow (function)
macro_rules! Depcrate_cacheprevent_overflow {
() => {
// Module: crate::cache
// Provides: {"prevent_overflow"}
// Dependencies: {}
fn prevent_overflow (atomic : & AtomicUsize , current : usize , upper : usize) { # [cold] fn cold_store (atomic : & AtomicUsize , upper : usize) { atomic . store (upper , Ordering :: Relaxed) ; } if current >= isize :: MAX as usize { cold_store (atomic , upper) ; } }
};
}
