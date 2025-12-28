macro_rules! deps {
    () => {
        IntoUrlSealed!();
        Result!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl IntoUrlSealed for String { fn into_url (self) -> crate :: Result < Url > { (& * self) . into_url () } fn as_str (& self) -> & str { self . as_ref () } }
    };
}

impl_45!();