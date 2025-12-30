// Generated macro for impl_85 (impl)
macro_rules! Depcrate_rt_cellimpl_85 {
() => {
// Module: crate::rt::cell
// Provides: {"impl_85"}
// Dependencies: {}
impl Drop for Writing { fn drop (& mut self) { rt :: execution (| execution | { let state = self . state . get_mut (& mut execution . objects) ; assert ! (state . is_writing) ; assert ! (state . is_reading == 0) ; state . is_writing = false ; if ! std :: thread :: panicking () { state . track_write (& execution . threads) ; } }) } }
};
}
