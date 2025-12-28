macro_rules! deps {
    () => {
        Crate!();
        HasCrate!();
        Function!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl HasCrate for Function { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_420!()