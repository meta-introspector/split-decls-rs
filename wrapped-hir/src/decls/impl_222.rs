macro_rules! deps {
    () => {
        HasContainer!();
        Const!();
        ItemContainer!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl HasContainer for Const { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
    };
}

impl_222!()