macro_rules! deps {
    () => {
        HasContainer!();
        Module!();
        ExternBlock!();
        ItemContainer!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl HasContainer for ExternBlock { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_225!()