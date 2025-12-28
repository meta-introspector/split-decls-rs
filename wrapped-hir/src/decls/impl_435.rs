macro_rules! deps {
    () => {
        Union!();
        HasContainer!();
        Module!();
        ItemContainer!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl HasContainer for Union { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_435!()