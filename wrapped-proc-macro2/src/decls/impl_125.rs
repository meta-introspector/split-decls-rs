macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < T > PartialEq < T > for Ident where T : ? Sized + AsRef < str > , { fn eq (& self , other : & T) -> bool { let other = other . as_ref () ; if self . raw { other . starts_with ("r#") && * self . sym == other [2 ..] } else { * self . sym == * other } } }
    };
}

impl_125!();