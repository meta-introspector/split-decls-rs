macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl PartialEq < & '_ str > for Label { fn eq (& self , other : & & str) -> bool { self == * other } }
    };
}

impl_85!();