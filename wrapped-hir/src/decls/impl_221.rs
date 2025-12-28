macro_rules! deps {
    () => {
        HasContainer!();
        ItemContainer!();
        TypeAlias!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl HasContainer for TypeAlias { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
    };
}

impl_221!()