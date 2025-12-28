macro_rules! deps {
    () => {
        Adt!();
        HasCrate!();
        Crate!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl HasCrate for Adt { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_427!();