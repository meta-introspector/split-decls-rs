// Generated macro for drop_future (function)
macro_rules! Depcrate_rawdrop_future {
() => {
// Module: crate::raw
// Provides: {"drop_future"}
// Dependencies: {}
# [doc = " Drops the future inside a task."] # [inline] unsafe fn drop_future < F > (ptr : * const () , task_layout : & TaskLayout) { let future_ptr = ptr . add_byte (task_layout . offset_f) as * mut F ; abort_on_panic (| | { future_ptr . drop_in_place () ; }) }
};
}
