macro_rules! VTabLog {
    () => {
        # [doc = " An instance of the vtablog virtual table"] # [repr (C)] struct VTabLog { # [doc = " Base class. Must be first"] base : ffi :: sqlite3_vtab , # [doc = " Associated connection"] db : * mut ffi :: sqlite3 , # [doc = " Number of rows in the table"] n_row : i64 , # [doc = " Instance number for this vtablog table"] i_inst : usize , # [doc = " Number of cursors created"] n_cursor : usize , }
    };
}

VTabLog!()