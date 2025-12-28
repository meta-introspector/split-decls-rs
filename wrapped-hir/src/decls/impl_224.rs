macro_rules! deps {
    () => {
        HasContainer!();
        Trait!();
        Module!();
        ItemContainer!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl HasContainer for Trait { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_224!()