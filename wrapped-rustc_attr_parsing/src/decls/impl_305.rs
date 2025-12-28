macro_rules! deps {
    () => {
        NameValueParser!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl NameValueParser { pub fn value_as_lit (& self) -> & MetaItemLit { & self . value } pub fn value_as_str (& self) -> Option < Symbol > { self . value_as_lit () . kind . str () } }
    };
}

impl_305!()