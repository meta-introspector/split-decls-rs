macro_rules! deps {
    () => {
        ModuleId!();
        BlockRelativeModuleId!();
        DefMap!();
        DefDatabase!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl BlockRelativeModuleId { fn def_map (self , db : & dyn DefDatabase , krate : Crate) -> & DefMap { self . into_module (krate) . def_map (db) } fn into_module (self , krate : Crate) -> ModuleId { ModuleId { krate , block : self . block , local_id : self . local_id } } fn is_block_module (self) -> bool { self . block . is_some () && self . local_id == DefMap :: ROOT } }
    };
}

impl_367!()