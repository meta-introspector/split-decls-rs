macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [cfg (feature = "loadable_extension")] impl From < ffi :: InitError > for Error { # [cold] fn from (err : ffi :: InitError) -> Self { Self :: InitError (err) } }
    };
}

impl_6!()