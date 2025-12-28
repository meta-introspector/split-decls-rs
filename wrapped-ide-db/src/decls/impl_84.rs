macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl PartialEq < str > for Label { fn eq (& self , other : & str) -> bool { self . 0 == other } }
    };
}

impl_84!();