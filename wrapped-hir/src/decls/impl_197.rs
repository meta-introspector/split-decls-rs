macro_rules! deps {
    () => {
        HasCrate!();
        Crate!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < T : hir_def :: HasModule > HasCrate for T { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () . into () } }
    };
}

impl_197!()