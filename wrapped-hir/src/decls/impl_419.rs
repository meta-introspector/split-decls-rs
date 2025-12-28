macro_rules! deps {
    () => {
        Crate!();
        Variant!();
        HasCrate!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl HasCrate for Variant { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_419!()