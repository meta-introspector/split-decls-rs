macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < T > PartialEq < T > for Ident where T : ? Sized + AsRef < str > , { fn eq (& self , other : & T) -> bool { self . inner == other } }
    };
}

impl_227!();