// Generated macro for impl_1023 (impl)
macro_rules! Depcrateimpl_1023 {
() => {
// Module: crate
// Provides: {"impl_1023"}
// Dependencies: {}
impl InterruptHandle { # [doc = " Interrupt the query currently executing on another thread. This will"] # [doc = " cause that query to fail with a `SQLITE3_INTERRUPT` error."] pub fn interrupt (& self) { let db_handle = self . db_lock . lock () . unwrap () ; if ! db_handle . is_null () { unsafe { ffi :: sqlite3_interrupt (* db_handle) } } } }
};
}
