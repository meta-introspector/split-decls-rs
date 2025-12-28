macro_rules! deps {
    () => {
        ExternCrateDecl!();
        HasVisibility!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl HasVisibility for ExternCrateDecl { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
    };
}

impl_295!()