macro_rules! deps {
    () => {
        HasContainer!();
        ItemContainer!();
        Module!();
        Enum!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl HasContainer for Enum { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
    };
}

impl_220!()