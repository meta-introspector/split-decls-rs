macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl From < i32 > for Action { # [inline] fn from (code : i32) -> Self { match code { ffi :: SQLITE_DELETE => Self :: SQLITE_DELETE , ffi :: SQLITE_INSERT => Self :: SQLITE_INSERT , ffi :: SQLITE_UPDATE => Self :: SQLITE_UPDATE , _ => Self :: UNKNOWN , } } }
    };
}

impl_126!();