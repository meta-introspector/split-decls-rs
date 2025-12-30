// Generated macro for owned_to_vec (function)
macro_rules! Depcrate_bytesowned_to_vec {
() => {
// Module: crate::bytes
// Provides: {"owned_to_vec"}
// Dependencies: {}
unsafe fn owned_to_vec < T > (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Vec < u8 > { let slice = slice :: from_raw_parts (ptr , len) ; let vec = slice . to_vec () ; owned_drop_impl :: < T > (data . load (Ordering :: Relaxed)) ; vec }
};
}
