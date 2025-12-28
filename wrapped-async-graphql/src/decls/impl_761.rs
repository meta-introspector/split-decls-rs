macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! impl_761 {
    () => {
        deps!();
        impl AsRef < str > for ID { fn as_ref (& self) -> & str { self . 0 . as_str () } }
    };
}

impl_761!()