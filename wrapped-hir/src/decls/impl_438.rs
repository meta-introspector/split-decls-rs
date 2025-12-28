macro_rules! deps {
    () => {
        Const!();
        HasContainer!();
        ItemContainer!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl HasContainer for Const { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
    };
}

impl_438!();