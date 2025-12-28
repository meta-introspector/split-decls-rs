macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for str { # [inline] fn as_ref (& self) -> & Utf8Path { Utf8Path :: new (self) } }
    };
}

impl_104!()