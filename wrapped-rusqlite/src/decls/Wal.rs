macro_rules! Wal {
    () => {
        # [doc = " Write-Ahead Log"] pub struct Wal { db : * mut ffi :: sqlite3 , db_name : * const c_char , }
    };
}

Wal!()