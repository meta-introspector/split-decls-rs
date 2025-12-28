macro_rules! deps {
    () => {
        ItemContainer!();
        HasContainer!();
        TypeAlias!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl HasContainer for TypeAlias { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
    };
}

impl_437!();