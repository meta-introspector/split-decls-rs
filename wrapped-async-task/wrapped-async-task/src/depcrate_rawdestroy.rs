// Generated macro for destroy (function)
macro_rules! Depcrate_rawdestroy {
() => {
// Module: crate::raw
// Provides: {"destroy"}
// Dependencies: {}
# [doc = " Cleans up task's resources and deallocates it."] # [doc = ""] # [doc = " The schedule function will be dropped, and the task will then get deallocated."] # [doc = " The task must be closed before this function is called."] # [inline] unsafe fn destroy < S , M > (ptr : * const ()) { let header = ptr as * const Header ; let task_layout = (* header) . vtable . layout_info ; let schedule = ptr . add_byte (task_layout . offset_s) ; abort_on_panic (| | { (ptr as * mut HeaderWithMetadata < M >) . drop_in_place () ; (schedule as * mut S) . drop_in_place () ; }) ; alloc :: alloc :: dealloc (ptr as * mut u8 , task_layout . layout) ; }
};
}
