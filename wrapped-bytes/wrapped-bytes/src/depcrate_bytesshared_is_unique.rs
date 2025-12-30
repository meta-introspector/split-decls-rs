// Generated macro for shared_is_unique (function)
macro_rules! Depcrate_bytesshared_is_unique {
() => {
// Module: crate::bytes
// Provides: {"shared_is_unique"}
// Dependencies: {}
pub (crate) unsafe fn shared_is_unique (data : & AtomicPtr < () >) -> bool { let shared = data . load (Ordering :: Acquire) ; let ref_cnt = (* shared . cast :: < Shared > ()) . ref_cnt . load (Ordering :: Relaxed) ; ref_cnt == 1 }
};
}
