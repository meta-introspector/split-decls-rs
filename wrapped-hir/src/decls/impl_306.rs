macro_rules! deps {
    () => {
        Trait!();
        HasVisibility!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl HasVisibility for Trait { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
    };
}

impl_306!()