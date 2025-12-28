macro_rules! deps {
    () => {
        StatementStatus!();
    };
}

macro_rules! stmt_status {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn stmt_status (ptr : * mut ffi :: sqlite3_stmt , status : StatementStatus , reset : bool ,) -> i32 { assert ! (! ptr . is_null ()) ; ffi :: sqlite3_stmt_status (ptr , status as i32 , reset as i32) }
    };
}

stmt_status!();