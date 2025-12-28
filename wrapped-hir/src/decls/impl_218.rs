macro_rules! deps {
    () => {
        HasContainer!();
        ItemContainer!();
        Module!();
        Struct!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl HasContainer for Struct { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_218!()