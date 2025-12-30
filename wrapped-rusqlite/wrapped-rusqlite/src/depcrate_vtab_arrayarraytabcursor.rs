// Generated macro for ArrayTabCursor (struct)
macro_rules! Depcrate_vtab_arrayArrayTabCursor {
() => {
// Module: crate::vtab::array
// Provides: {"ArrayTabCursor"}
// Dependencies: {}
# [doc = " A cursor for the Array virtual table"] # [repr (C)] struct ArrayTabCursor < 'vtab > { # [doc = " Base class. Must be first"] base : ffi :: sqlite3_vtab_cursor , # [doc = " The rowid"] row_id : i64 , # [doc = " Pointer to the array of values (\"pointer\")"] ptr : Option < Array > , phantom : PhantomData < & 'vtab ArrayTab > , }
};
}
