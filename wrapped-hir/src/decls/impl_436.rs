macro_rules! deps {
    () => {
        HasContainer!();
        ItemContainer!();
        Module!();
        Enum!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl HasContainer for Enum { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_436!()