// Generated macro for impl_38 (impl)
macro_rules! Depcrate_connectionimpl_38 {
() => {
// Module: crate::connection
// Provides: {"impl_38"}
// Dependencies: {}
impl Drop for ConnectionRef { fn drop (& mut self) { let conn = & mut * self . state . lock ("drop") ; if let Some (x) = conn . ref_count . checked_sub (1) { conn . ref_count = x ; if x == 0 && ! conn . inner . is_closed () { conn . implicit_close (& self . shared) ; } } } }
};
}
