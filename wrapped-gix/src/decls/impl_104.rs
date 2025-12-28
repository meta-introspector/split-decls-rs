macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl From < std :: convert :: Infallible > for Error { fn from (_value : Infallible) -> Self { unreachable ! ("cannot be invoked") } }
    };
}

impl_104!()