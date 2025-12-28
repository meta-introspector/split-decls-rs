macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl From < csv :: Error > for Error { # [cold] fn from (err : csv :: Error) -> Self { Self :: ModuleError (err . to_string ()) } }
    };
}

impl_628!();