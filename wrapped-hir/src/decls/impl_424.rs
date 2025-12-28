macro_rules! deps {
    () => {
        HasCrate!();
        Crate!();
        Macro!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl HasCrate for Macro { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_424!();