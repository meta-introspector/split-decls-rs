// Generated macro for VTabLogCursor (struct)
macro_rules! Depcrate_vtab_vtablogVTabLogCursor {
() => {
// Module: crate::vtab::vtablog
// Provides: {"VTabLogCursor"}
// Dependencies: {}
# [doc = " A cursor for the Series virtual table"] # [repr (C)] struct VTabLogCursor < 'vtab > { # [doc = " Base class. Must be first"] base : ffi :: sqlite3_vtab_cursor , # [doc = " Cursor number"] i_cursor : usize , # [doc = " The rowid"] row_id : i64 , phantom : PhantomData < & 'vtab VTabLog > , }
};
}
