macro_rules! deps {
    () => {
        HasCrate!();
        Trait!();
        Crate!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl HasCrate for Trait { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_425!();