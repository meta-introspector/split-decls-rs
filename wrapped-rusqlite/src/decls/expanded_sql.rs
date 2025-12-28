macro_rules! deps {
    () => {
        SqliteMallocString!();
    };
}

macro_rules! expanded_sql {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn expanded_sql (ptr : * mut ffi :: sqlite3_stmt) -> Option < SqliteMallocString > { SqliteMallocString :: from_raw (ffi :: sqlite3_expanded_sql (ptr)) }
    };
}

expanded_sql!();