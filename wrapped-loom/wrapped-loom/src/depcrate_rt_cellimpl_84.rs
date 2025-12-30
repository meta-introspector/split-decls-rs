// Generated macro for impl_84 (impl)
macro_rules! Depcrate_rt_cellimpl_84 {
() => {
// Module: crate::rt::cell
// Provides: {"impl_84"}
// Dependencies: {}
impl Drop for Reading { fn drop (& mut self) { rt :: execution (| execution | { let state = self . state . get_mut (& mut execution . objects) ; assert ! (state . is_reading > 0) ; assert ! (! state . is_writing) ; state . is_reading -= 1 ; if ! std :: thread :: panicking () { state . track_read (& execution . threads) ; } }) } }
};
}
