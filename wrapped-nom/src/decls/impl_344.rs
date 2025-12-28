macro_rules! deps {
    () => {
        ParseTo!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl < 'a , R : FromStr > ParseTo < R > for & 'a [u8] { fn parse_to (& self) -> Option < R > { from_utf8 (self) . ok () . and_then (| s | s . parse () . ok ()) } }
    };
}

impl_344!()