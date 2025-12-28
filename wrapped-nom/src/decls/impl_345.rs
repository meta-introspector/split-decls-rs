macro_rules! deps {
    () => {
        ParseTo!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < 'a , R : FromStr > ParseTo < R > for & 'a str { fn parse_to (& self) -> Option < R > { self . parse () . ok () } }
    };
}

impl_345!();