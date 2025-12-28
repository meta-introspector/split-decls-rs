macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl AsRef < std :: path :: Path > for OsStr { # [inline] fn as_ref (& self) -> & std :: path :: Path { std :: path :: Path :: new (self) } }
    };
}

impl_124!()