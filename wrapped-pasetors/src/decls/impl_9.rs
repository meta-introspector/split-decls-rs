macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl From < ct_codecs :: Error > for Error { fn from (_ : ct_codecs :: Error) -> Self { Error :: Base64 } }
    };
}

impl_9!()