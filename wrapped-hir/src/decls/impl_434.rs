macro_rules! deps {
    () => {
        ItemContainer!();
        Struct!();
        Module!();
        HasContainer!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl HasContainer for Struct { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_434!();