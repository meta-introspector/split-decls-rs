macro_rules! deps {
    () => {
        Error!();
        ErrorExtensions!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl ErrorExtensions for Error { fn extend (& self) -> Error { self . clone () } }
    };
}

impl_53!()