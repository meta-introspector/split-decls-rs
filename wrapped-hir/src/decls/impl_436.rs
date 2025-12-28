macro_rules! deps {
    () => {
        Module!();
        Enum!();
        HasContainer!();
        ItemContainer!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl HasContainer for Enum { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_436!();