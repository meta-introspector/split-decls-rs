// Generated macro for promotable_odd_clone (function)
macro_rules! Depcrate_bytespromotable_odd_clone {
() => {
// Module: crate::bytes
// Provides: {"promotable_odd_clone"}
// Dependencies: {}
unsafe fn promotable_odd_clone (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Bytes { let shared = data . load (Ordering :: Acquire) ; let kind = shared as usize & KIND_MASK ; if kind == KIND_ARC { shallow_clone_arc (shared as _ , ptr , len) } else { debug_assert_eq ! (kind , KIND_VEC) ; shallow_clone_vec (data , shared , shared . cast () , ptr , len) } }
};
}
