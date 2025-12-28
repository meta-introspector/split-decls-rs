macro_rules! deps {
    () => {
        Module!();
        Union!();
        HasContainer!();
        ItemContainer!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl HasContainer for Union { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_219!()