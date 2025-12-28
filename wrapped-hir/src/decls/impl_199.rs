macro_rules! deps {
    () => {
        HasCrate!();
        Struct!();
        Crate!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl HasCrate for Struct { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_199!()