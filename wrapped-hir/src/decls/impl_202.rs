macro_rules! deps {
    () => {
        HasCrate!();
        Crate!();
        Field!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl HasCrate for Field { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . parent_def (db) . module (db) . krate () } }
    };
}

impl_202!()