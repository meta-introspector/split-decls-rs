macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_1068 {
    () => {
        deps!();
        impl From < read :: Error > for Error { fn from (error : read :: Error) -> Error { Error (format ! ("{}" , error)) } }
    };
}

impl_1068!()