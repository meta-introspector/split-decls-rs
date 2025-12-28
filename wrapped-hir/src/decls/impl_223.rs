macro_rules! deps {
    () => {
        HasContainer!();
        ItemContainer!();
        Static!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl HasContainer for Static { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
    };
}

impl_223!()