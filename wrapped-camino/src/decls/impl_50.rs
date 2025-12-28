macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl AsRef < Path > for Iter < '_ > { # [inline] fn as_ref (& self) -> & Path { self . as_path () . as_ref () } }
    };
}

impl_50!()