macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl From < Infallible > for Error { fn from (_ : Infallible) -> Self { unreachable ! ("Cannot actually happen, but it seems there can't be a blanket impl for this") } }
    };
}

impl_24!()