macro_rules! deps {
    () => {
        Struct!();
        HasVisibility!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl HasVisibility for Struct { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
    };
}

impl_39!()