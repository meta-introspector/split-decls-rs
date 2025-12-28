macro_rules! deps {
    () => {
        HasVisibility!();
        Trait!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl HasVisibility for Trait { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
    };
}

impl_90!()