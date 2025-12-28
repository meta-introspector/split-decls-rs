macro_rules! deps {
    () => {
        HasCrate!();
        Crate!();
        TypeAlias!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl HasCrate for TypeAlias { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_206!()