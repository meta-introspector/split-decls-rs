macro_rules! deps {
    () => {
        HasContainer!();
        ItemContainer!();
        ExternCrateDecl!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl HasContainer for ExternCrateDecl { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container . into ()) } }
    };
}

impl_431!()