macro_rules! deps {
    () => {
        Function!();
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl HasCrate for Function { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_420!();