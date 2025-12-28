macro_rules! deps {
    () => {
        Crate!();
        HasCrate!();
        Struct!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        impl HasCrate for Struct { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_415!()