macro_rules! deps {
    () => {
        Module!();
        ItemContainer!();
        HasContainer!();
        ExternBlock!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        impl HasContainer for ExternBlock { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_441!()