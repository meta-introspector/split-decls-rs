macro_rules! get_autocommit {
    () => {
        # [inline] pub (crate) unsafe fn get_autocommit (ptr : * mut ffi :: sqlite3) -> bool { ffi :: sqlite3_get_autocommit (ptr) != 0 }
    };
}

get_autocommit!()