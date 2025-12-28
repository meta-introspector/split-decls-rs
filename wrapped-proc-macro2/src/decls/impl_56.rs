macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl PartialEq for Ident { fn eq (& self , other : & Ident) -> bool { self . inner == other . inner } }
    };
}

impl_56!()