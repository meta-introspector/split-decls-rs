macro_rules! deps {
    () => {
        IntoUrlSealed!();
        Result!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < 'a > IntoUrlSealed for & 'a str { fn into_url (self) -> crate :: Result < Url > { Url :: parse (self) . map_err (crate :: error :: builder) ? . into_url () } fn as_str (& self) -> & str { self } }
    };
}

impl_43!()