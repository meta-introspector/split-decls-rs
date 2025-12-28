macro_rules! deps {
    () => {
        Enum!();
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl HasCrate for Enum { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_417!()