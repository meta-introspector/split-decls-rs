macro_rules! deps {
    () => {
        Crate!();
        HasCrate!();
        Impl!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl HasCrate for Impl { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_212!()