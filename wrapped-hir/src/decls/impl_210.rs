macro_rules! deps {
    () => {
        HasCrate!();
        Crate!();
        Static!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl HasCrate for Static { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_210!()