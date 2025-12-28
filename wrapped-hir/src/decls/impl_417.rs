macro_rules! deps {
    () => {
        Crate!();
        Enum!();
        HasCrate!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl HasCrate for Enum { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_417!();