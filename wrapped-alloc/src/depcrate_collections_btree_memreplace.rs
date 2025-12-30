// Generated macro for replace (function)
macro_rules! Depcrate_collections_btree_memreplace {
() => {
// Module: crate::collections::btree::mem
// Provides: {"replace"}
// Dependencies: {}
# [doc = " This replaces the value behind the `v` unique reference by calling the"] # [doc = " relevant function, and returns a result obtained along the way."] # [doc = ""] # [doc = " If a panic occurs in the `change` closure, the entire process will be aborted."] # [inline] pub (super) fn replace < T , R > (v : & mut T , change : impl FnOnce (T) -> (T , R)) -> R { struct PanicGuard ; impl Drop for PanicGuard { fn drop (& mut self) { intrinsics :: abort () } } let guard = PanicGuard ; let value = unsafe { ptr :: read (v) } ; let (new_value , ret) = change (value) ; unsafe { ptr :: write (v , new_value) ; } mem :: forget (guard) ; ret }
};
}
