macro_rules! deps {
    () => {
        Authorization!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl Authorization { fn into_raw (self) -> c_int { match self { Self :: Allow => ffi :: SQLITE_OK , Self :: Ignore => ffi :: SQLITE_IGNORE , Self :: Deny => ffi :: SQLITE_DENY , } } }
    };
}

impl_134!()