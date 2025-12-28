macro_rules! deps {
    () => {
        Macro!();
        HasCrate!();
        Crate!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl HasCrate for Macro { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_424!()