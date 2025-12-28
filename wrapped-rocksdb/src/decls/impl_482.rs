macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl AsRef < str > for Error { fn as_ref (& self) -> & str { & self . message } }
    };
}

impl_482!();