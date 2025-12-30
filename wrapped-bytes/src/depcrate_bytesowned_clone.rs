// Generated macro for owned_clone (function)
macro_rules! Depcrate_bytesowned_clone {
() => {
// Module: crate::bytes
// Provides: {"owned_clone"}
// Dependencies: {}
unsafe fn owned_clone < T > (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Bytes { let owned = data . load (Ordering :: Relaxed) ; let old_cnt = (* owned . cast :: < AtomicUsize > ()) . fetch_add (1 , Ordering :: Relaxed) ; if old_cnt > usize :: MAX >> 1 { crate :: abort () ; } Bytes { ptr , len , data : AtomicPtr :: new (owned as _) , vtable : & Owned :: < T > :: VTABLE , } }
};
}
