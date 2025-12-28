macro_rules! deps {
    () => {
        TypeAlias!();
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl HasCrate for TypeAlias { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_422!()