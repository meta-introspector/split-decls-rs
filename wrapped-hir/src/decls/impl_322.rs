macro_rules! deps {
    () => {
        ItemInNs!();
        Macro!();
        Crate!();
        ModuleDef!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl ItemInNs { pub fn into_module_def (self) -> ModuleDef { match self { ItemInNs :: Types (id) | ItemInNs :: Values (id) => id , ItemInNs :: Macros (id) => ModuleDef :: Macro (id) , } } # [doc = " Returns the crate defining this item (or `None` if `self` is built-in)."] pub fn krate (& self , db : & dyn HirDatabase) -> Option < Crate > { match self { ItemInNs :: Types (did) | ItemInNs :: Values (did) => did . module (db) . map (| m | m . krate ()) , ItemInNs :: Macros (id) => Some (id . module (db) . krate ()) , } } pub fn attrs (& self , db : & dyn HirDatabase) -> Option < AttrsWithOwner > { match self { ItemInNs :: Types (it) | ItemInNs :: Values (it) => it . attrs (db) , ItemInNs :: Macros (it) => Some (it . attrs (db)) , } } }
    };
}

impl_322!()