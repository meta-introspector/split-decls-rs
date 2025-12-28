macro_rules! deps {
    () => {
        HasVisibility!();
        Union!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl HasVisibility for Union { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
    };
}

impl_260!();