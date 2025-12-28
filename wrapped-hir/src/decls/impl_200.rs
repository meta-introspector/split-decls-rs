macro_rules! deps {
    () => {
        HasCrate!();
        Crate!();
        Union!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl HasCrate for Union { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_200!()