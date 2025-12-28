macro_rules! deps {
    () => {
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl < T : hir_def :: HasModule > HasCrate for T { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () . into () } }
    };
}

impl_413!()