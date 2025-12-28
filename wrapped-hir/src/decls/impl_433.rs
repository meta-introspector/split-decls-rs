macro_rules! deps {
    () => {
        HasContainer!();
        ItemContainer!();
        Function!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl HasContainer for Function { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
    };
}

impl_433!();