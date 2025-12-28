macro_rules! deps {
    () => {
        ValueRef!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for ValueRef < 'a > { # [inline] fn from (s : & str) -> ValueRef < '_ > { ValueRef :: Text (s . as_bytes ()) } }
    };
}

impl_505!()