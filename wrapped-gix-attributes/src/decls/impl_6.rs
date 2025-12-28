macro_rules! deps {
    () => {
        NameRef!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl AsRef < str > for NameRef < '_ > { fn as_ref (& self) -> & str { self . 0 . as_ref () } }
    };
}

impl_6!();