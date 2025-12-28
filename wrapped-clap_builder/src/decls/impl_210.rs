macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl AsRef < std :: path :: Path > for Str { # [inline] fn as_ref (& self) -> & std :: path :: Path { std :: path :: Path :: new (self) } }
    };
}

impl_210!();