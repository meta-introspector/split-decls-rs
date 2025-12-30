// Generated macro for impl_38 (impl)
macro_rules! Depcrate_rawimpl_38 {
() => {
// Module: crate::raw
// Provides: {"impl_38"}
// Dependencies: {}
impl TaskVTable { # [doc = " Returns a pointer to the output inside a task."] pub (crate) unsafe fn get_output (& self , ptr : * const ()) -> * const () { ptr . add_byte (self . layout_info . offset_r) } }
};
}
