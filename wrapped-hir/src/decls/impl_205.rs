macro_rules! deps {
    () => {
        Const!();
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl HasCrate for Const { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_205!()