macro_rules! deps {
    () => {
        Result!();
        Filters!();
        Context!();
    };
}

macro_rules! VTabCursor {
    () => {
        deps!();
        # [doc = " Virtual table cursor trait."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations must be like:"] # [doc = " ```rust,ignore"] # [doc = " #[repr(C)]"] # [doc = " struct MyTabCursor {"] # [doc = "    /// Base class. Must be first"] # [doc = "    base: rusqlite::vtab::sqlite3_vtab_cursor,"] # [doc = "    /* Virtual table implementations will typically add additional fields */"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " (See [SQLite doc](https://sqlite.org/c3ref/vtab_cursor.html))"] pub unsafe trait VTabCursor : Sized { # [doc = " Begin a search of a virtual table."] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#the_xfilter_method))"] fn filter (& mut self , idx_num : c_int , idx_str : Option < & str > , args : & Filters < '_ >) -> Result < () > ; # [doc = " Advance cursor to the next row of a result set initiated by"] # [doc = " [`filter`](VTabCursor::filter). (See [SQLite doc](https://sqlite.org/vtab.html#the_xnext_method))"] fn next (& mut self) -> Result < () > ; # [doc = " Must return `false` if the cursor currently points to a valid row of"] # [doc = " data, or `true` otherwise."] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#the_xeof_method))"] fn eof (& self) -> bool ; # [doc = " Find the value for the `i`-th column of the current row."] # [doc = " `i` is zero-based so the first column is numbered 0."] # [doc = " May return its result back to SQLite using one of the specified `ctx`."] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#the_xcolumn_method))"] fn column (& self , ctx : & mut Context , i : c_int) -> Result < () > ; # [doc = " Return the rowid of row that the cursor is currently pointing at."] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#the_xrowid_method))"] fn rowid (& self) -> Result < i64 > ; }
    };
}

VTabCursor!();