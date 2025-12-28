macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Ident { pub fn as_str (& self) -> & str { & self . name } }
    };
}

impl_7!()