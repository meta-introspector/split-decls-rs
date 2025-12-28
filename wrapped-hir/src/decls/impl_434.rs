macro_rules! deps {
    () => {
        Module!();
        HasContainer!();
        Struct!();
        ItemContainer!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl HasContainer for Struct { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_434!()