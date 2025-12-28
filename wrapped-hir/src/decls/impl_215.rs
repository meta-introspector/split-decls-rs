macro_rules! deps {
    () => {
        ExternCrateDecl!();
        ItemContainer!();
        HasContainer!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl HasContainer for ExternCrateDecl { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container . into ()) } }
    };
}

impl_215!()