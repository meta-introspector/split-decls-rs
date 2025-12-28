macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for & 'a Utf8Path { fn from (s : & 'a str) -> & 'a Utf8Path { Utf8Path :: new (s) } }
    };
}

impl_76!();