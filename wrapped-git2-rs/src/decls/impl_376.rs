macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_376 {
    () => {
        deps!();
        impl From < NulError > for Error { fn from (_ : NulError) -> Error { Error :: from_str ("data contained a nul byte that could not be \
             represented as a string" ,) } }
    };
}

impl_376!()