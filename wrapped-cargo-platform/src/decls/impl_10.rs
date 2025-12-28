macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl PartialEq < & str > for Ident { fn eq (& self , other : & & str) -> bool { self . name == * other } }
    };
}

impl_10!();