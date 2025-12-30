// Generated macro for drop_waker (function)
macro_rules! Depcrate_rawdrop_waker {
() => {
// Module: crate::raw
// Provides: {"drop_waker"}
// Dependencies: {}
# [doc = " Drops a waker."] # [doc = ""] # [doc = " This function will decrement the reference count. If it drops down to zero, the associated"] # [doc = " `Task` has been dropped too, and the task has not been completed, then it will get"] # [doc = " scheduled one more time so that its future gets dropped by the executor."] # [inline] unsafe fn drop_waker (ptr : * const ()) { let header = ptr as * const Header ; match Header :: drop_waker (ptr) { DropWakerAction :: Schedule => ((* header) . vtable . schedule) (ptr , ScheduleInfo :: new (false)) , DropWakerAction :: Destroy => ((* header) . vtable . destroy) (ptr) , DropWakerAction :: None => { } } }
};
}
