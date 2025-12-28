macro_rules! deps {
    () => {
        ItemContainer!();
        HasContainer!();
        Module!();
        Trait!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        impl HasContainer for Trait { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_440!();