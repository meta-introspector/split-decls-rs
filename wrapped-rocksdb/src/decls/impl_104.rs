macro_rules! deps {
    () => {
        DBWithThreadModeInner!();
        DBInner!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl DBInner for DBWithThreadModeInner { fn inner (& self) -> * mut ffi :: rocksdb_t { self . inner } }
    };
}

impl_104!();