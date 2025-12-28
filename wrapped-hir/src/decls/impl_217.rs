macro_rules! deps {
    () => {
        Function!();
        HasContainer!();
        ItemContainer!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl HasContainer for Function { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
    };
}

impl_217!()