macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl PartialEq for Ident { fn eq (& self , other : & Ident) -> bool { self . sym == other . sym && self . raw == other . raw } }
    };
}

impl_124!();