// Generated macro for drop_ref (function)
macro_rules! Depcrate_rawdrop_ref {
() => {
// Module: crate::raw
// Provides: {"drop_ref"}
// Dependencies: {}
# [doc = " Drops a task reference (`Runnable` or `Waker`)."] # [doc = ""] # [doc = " This function will decrement the reference count. If it drops down to zero and the"] # [doc = " associated `Task` handle has been dropped too, then the task gets destroyed."] # [inline] pub (crate) unsafe fn drop_ref (ptr : * const ()) { let header = ptr as * const Header ; let header = & * header ; let new = header . state . fetch_sub (REFERENCE , Ordering :: AcqRel) - REFERENCE ; if new & ! (REFERENCE - 1) == 0 && new & TASK == 0 { (header . vtable . destroy) (ptr) ; } }
};
}
