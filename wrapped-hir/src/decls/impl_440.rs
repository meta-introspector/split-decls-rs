macro_rules! deps {
    () => {
        Module!();
        HasContainer!();
        Trait!();
        ItemContainer!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        impl HasContainer for Trait { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_440!()