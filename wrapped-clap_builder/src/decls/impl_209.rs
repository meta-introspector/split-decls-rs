macro_rules! deps {
    () => {
        OsStr!();
        Str!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl AsRef < std :: ffi :: OsStr > for Str { # [inline] fn as_ref (& self) -> & std :: ffi :: OsStr { (* * self) . as_ref () } }
    };
}

impl_209!();