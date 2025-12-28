macro_rules! deps {
    () => {
        Array!();
        ArrayTab!();
    };
}

macro_rules! ArrayTabCursor {
    () => {
        deps!();
        # [doc = " A cursor for the Array virtual table"] # [repr (C)] struct ArrayTabCursor < 'vtab > { # [doc = " Base class. Must be first"] base : ffi :: sqlite3_vtab_cursor , # [doc = " The rowid"] row_id : i64 , # [doc = " Pointer to the array of values (\"pointer\")"] ptr : Option < Array > , phantom : PhantomData < & 'vtab ArrayTab > , }
    };
}

ArrayTabCursor!();