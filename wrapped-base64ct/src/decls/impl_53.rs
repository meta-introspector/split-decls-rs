macro_rules! deps {
    () => {
        InvalidLengthError!();
        Error!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl From < InvalidLengthError > for Error { # [inline] fn from (_ : InvalidLengthError) -> Error { Error :: InvalidLength } }
    };
}

impl_53!();