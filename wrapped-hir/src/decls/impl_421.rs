macro_rules! deps {
    () => {
        HasCrate!();
        Crate!();
        Const!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        impl HasCrate for Const { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_421!();