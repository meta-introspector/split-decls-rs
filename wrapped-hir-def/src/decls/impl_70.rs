macro_rules! deps {
    () => {
        ItemInNs!();
        DefDatabase!();
        ModuleId!();
        ModuleDefId!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl ItemInNs { pub fn as_module_def_id (self) -> Option < ModuleDefId > { match self { ItemInNs :: Types (id) | ItemInNs :: Values (id) => Some (id) , ItemInNs :: Macros (_) => None , } } # [doc = " Returns the crate defining this item (or `None` if `self` is built-in)."] pub fn krate (& self , db : & dyn DefDatabase) -> Option < Crate > { match self { ItemInNs :: Types (id) | ItemInNs :: Values (id) => id . module (db) . map (| m | m . krate) , ItemInNs :: Macros (id) => Some (id . module (db) . krate) , } } pub fn module (& self , db : & dyn DefDatabase) -> Option < ModuleId > { match self { ItemInNs :: Types (id) | ItemInNs :: Values (id) => id . module (db) , ItemInNs :: Macros (id) => Some (id . module (db)) , } } }
    };
}

impl_70!();