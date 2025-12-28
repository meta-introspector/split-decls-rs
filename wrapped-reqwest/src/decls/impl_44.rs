macro_rules! deps {
    () => {
        IntoUrlSealed!();
        Result!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'a > IntoUrlSealed for & 'a String { fn into_url (self) -> crate :: Result < Url > { (& * * self) . into_url () } fn as_str (& self) -> & str { self . as_ref () } }
    };
}

impl_44!()