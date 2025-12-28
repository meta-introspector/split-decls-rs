macro_rules! deps {
    () => {
        OwnedData!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl Drop for OwnedData { fn drop (& mut self) { unsafe { ffi :: sqlite3_free (self . ptr . as_ptr () . cast ()) ; } } }
    };
}

impl_238!()