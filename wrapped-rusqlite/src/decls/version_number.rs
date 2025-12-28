macro_rules! version_number {
    () => {
        # [doc = " Returns the SQLite version as an integer; e.g., `3016002` for version"] # [doc = " 3.16.2."] # [doc = ""] # [doc = " See [`sqlite3_libversion_number()`](https://www.sqlite.org/c3ref/libversion.html)."] # [inline] # [must_use] pub fn version_number () -> i32 { unsafe { ffi :: sqlite3_libversion_number () } }
    };
}

version_number!();