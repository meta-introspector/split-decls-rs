// Generated macro for impl_3736 (impl)
macro_rules! Depcrate_sqlite_connection_rawimpl_3736 {
() => {
// Module: crate::sqlite::connection::raw
// Provides: {"impl_3736"}
// Dependencies: {}
impl SqliteCallbackError { fn emit (& self , ctx : * mut ffi :: sqlite3_context) { let s ; let msg = match self { SqliteCallbackError :: Abort (msg) => * msg , SqliteCallbackError :: DieselError (e) => { s = e . to_string () ; & s } SqliteCallbackError :: Panic (msg) => msg , } ; unsafe { context_error_str (ctx , msg) ; } } }
};
}
