macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl PartialEq < Ident > for Ident { fn eq (& self , other : & Ident) -> bool { self . name == other . name } }
    };
}

impl_11!()