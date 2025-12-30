// Generated macro for owned_drop (function)
macro_rules! Depcrate_bytesowned_drop {
() => {
// Module: crate::bytes
// Provides: {"owned_drop"}
// Dependencies: {}
unsafe fn owned_drop < T > (data : & mut AtomicPtr < () > , _ptr : * const u8 , _len : usize) { let owned = data . load (Ordering :: Relaxed) ; owned_drop_impl :: < T > (owned) ; }
};
}
