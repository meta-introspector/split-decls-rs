// Generated macro for promotable_is_unique (function)
macro_rules! Depcrate_bytespromotable_is_unique {
() => {
// Module: crate::bytes
// Provides: {"promotable_is_unique"}
// Dependencies: {}
unsafe fn promotable_is_unique (data : & AtomicPtr < () >) -> bool { let shared = data . load (Ordering :: Acquire) ; let kind = shared as usize & KIND_MASK ; if kind == KIND_ARC { let ref_cnt = (* shared . cast :: < Shared > ()) . ref_cnt . load (Ordering :: Relaxed) ; ref_cnt == 1 } else { true } }
};
}
