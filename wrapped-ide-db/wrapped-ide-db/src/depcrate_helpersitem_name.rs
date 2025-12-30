// Generated macro for item_name (function)
macro_rules! Depcrate_helpersitem_name {
() => {
// Module: crate::helpers
// Provides: {"item_name"}
// Dependencies: {}
pub fn item_name (db : & RootDatabase , item : ItemInNs) -> Option < Name > { match item { ItemInNs :: Types (module_def_id) => module_def_id . name (db) , ItemInNs :: Values (module_def_id) => module_def_id . name (db) , ItemInNs :: Macros (macro_def_id) => Some (macro_def_id . name (db)) , } }
};
}
