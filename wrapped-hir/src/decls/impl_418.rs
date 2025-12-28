macro_rules! deps {
    () => {
        Field!();
        HasCrate!();
        Crate!();
    };
}

macro_rules! impl_418 {
    () => {
        deps!();
        impl HasCrate for Field { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . parent_def (db) . module (db) . krate () } }
    };
}

impl_418!()