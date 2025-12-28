macro_rules! deps {
    () => {
        Trait!();
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl HasCrate for Trait { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_209!()