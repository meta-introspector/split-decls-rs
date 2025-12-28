macro_rules! deps {
    () => {
        HasVisibility!();
        Field!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl HasVisibility for Field { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let variant_data = VariantId :: from (self . parent) . fields (db) ; let visibility = & variant_data . fields () [self . id] . visibility ; let parent_id : hir_def :: VariantId = self . parent . into () ; Visibility :: resolve (db , & parent_id . resolver (db) , visibility) } }
    };
}

impl_252!()