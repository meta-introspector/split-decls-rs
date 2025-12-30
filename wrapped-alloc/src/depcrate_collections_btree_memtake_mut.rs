// Generated macro for take_mut (function)
macro_rules! Depcrate_collections_btree_memtake_mut {
() => {
// Module: crate::collections::btree::mem
// Provides: {"take_mut"}
// Dependencies: {}
# [doc = " This replaces the value behind the `v` unique reference by calling the"] # [doc = " relevant function."] # [doc = ""] # [doc = " If a panic occurs in the `change` closure, the entire process will be aborted."] # [allow (dead_code)] # [inline] pub (super) fn take_mut < T > (v : & mut T , change : impl FnOnce (T) -> T) { replace (v , | value | (change (value) , ())) }
};
}
