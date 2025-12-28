macro_rules! deps {
    () => {
        Error!();
        InvalidEncodingError!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl From < InvalidEncodingError > for Error { # [inline] fn from (_ : InvalidEncodingError) -> Error { Error :: InvalidEncoding } }
    };
}

impl_52!();