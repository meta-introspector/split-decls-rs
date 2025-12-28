macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl AsRef < str > for Iter < '_ > { # [inline] fn as_ref (& self) -> & str { self . as_path () . as_ref () } }
    };
}

impl_34!()