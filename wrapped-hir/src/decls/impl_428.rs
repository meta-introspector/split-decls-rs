macro_rules! deps {
    () => {
        HasCrate!();
        Impl!();
        Crate!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl HasCrate for Impl { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_428!();