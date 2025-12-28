macro_rules! deps {
    () => {
        DefDatabase!();
        ItemLoc!();
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_727 {
    () => {
        deps!();
        impl < N , ItemId > HasModule for ItemId where N : AstIdNode , ItemId : Lookup < Database = dyn DefDatabase , Data = ItemLoc < N > > + Copy , { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . container } }
    };
}

impl_727!();