macro_rules! deps {
    () => {
        HasCrate!();
        Crate!();
        Variant!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl HasCrate for Variant { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_419!();