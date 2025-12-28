macro_rules! deps {
    () => {
        Adt!();
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl HasCrate for Adt { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_211!()