macro_rules! deps {
    () => {
        DBWithThreadModeInner!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl Drop for DBWithThreadModeInner { fn drop (& mut self) { unsafe { ffi :: rocksdb_close (self . inner) ; } } }
    };
}

impl_105!()