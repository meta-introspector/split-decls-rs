macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl AsRef < str > for Iter < '_ > { # [inline] fn as_ref (& self) -> & str { self . as_path () . as_ref () } }
    };
}

impl_51!();