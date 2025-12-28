macro_rules! deps {
    () => {
        Trait!();
        HasCrate!();
        Crate!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl HasCrate for Trait { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_425!()