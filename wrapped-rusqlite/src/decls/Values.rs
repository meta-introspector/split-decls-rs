macro_rules! Values {
    () => {
        # [doc = " Wrapper to [ffi::sqlite3_value]s"] pub struct Values < 'a > { args : & 'a [* mut ffi :: sqlite3_value] , }
    };
}

Values!();