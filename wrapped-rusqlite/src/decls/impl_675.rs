macro_rules! deps {
    () => {
        SqliteMallocString!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        impl Drop for SqliteMallocString { # [inline] fn drop (& mut self) { unsafe { ffi :: sqlite3_free (self . ptr . as_ptr () . cast ()) } ; } }
    };
}

impl_675!();