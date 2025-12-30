// Generated macro for parse_with_cbs (function)
macro_rules! Depcrateparse_with_cbs {
() => {
// Module: crate
// Provides: {"parse_with_cbs"}
// Dependencies: {}
# [doc = " Calls `parse_func` with a `CBS` structure pointing at `data`."] # [doc = " If that returns a null pointer then it returns [None]."] # [doc = " Otherwise, if there's still data left in CBS, it calls `free_func` on the"] # [doc = " pointer and returns [None]. Otherwise it returns the pointer."] fn parse_with_cbs < T , Parse , Free > (data : & [u8] , free_func : Free , parse_func : Parse) -> Option < * mut T > where Parse : FnOnce (* mut bssl_sys :: CBS) -> * mut T , Free : FnOnce (* mut T) , { let mut cbs = unsafe { initialized_struct (| cbs | bssl_sys :: CBS_init (cbs , data . as_ffi_ptr () , data . len ())) } ; let ptr = parse_func (& mut cbs) ; if ptr . is_null () { return None ; } if unsafe { bssl_sys :: CBS_len (& cbs) } != 0 { free_func (ptr) ; return None ; } Some (ptr) }
};
}
