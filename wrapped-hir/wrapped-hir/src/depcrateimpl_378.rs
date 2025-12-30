// Generated macro for impl_378 (impl)
macro_rules! Depcrateimpl_378 {
() => {
// Module: crate
// Provides: {"impl_378"}
// Dependencies: {}
impl ExternCrateDecl { pub fn module (self , db : & dyn HirDatabase) -> Module { self . id . module (db) . into () } pub fn resolved_crate (self , db : & dyn HirDatabase) -> Option < Crate > { let loc = self . id . lookup (db) ; let krate = loc . container . krate () ; let name = self . name (db) ; if name == sym :: self_ { Some (krate . into ()) } else { krate . data (db) . dependencies . iter () . find_map (| dep | { if dep . name . symbol () == name . symbol () { Some (dep . crate_id . into ()) } else { None } }) } } pub fn name (self , db : & dyn HirDatabase) -> Name { let loc = self . id . lookup (db) ; let source = loc . source (db) ; as_name_opt (source . value . name_ref ()) } pub fn alias (self , db : & dyn HirDatabase) -> Option < ImportAlias > { let loc = self . id . lookup (db) ; let source = loc . source (db) ; let rename = source . value . rename () ? ; if let Some (name) = rename . name () { Some (ImportAlias :: Alias (name . as_name ())) } else if rename . underscore_token () . is_some () { Some (ImportAlias :: Underscore) } else { None } } # [doc = " Returns the name under which this crate is made accessible, taking `_` into account."] pub fn alias_or_name (self , db : & dyn HirDatabase) -> Option < Name > { match self . alias (db) { Some (ImportAlias :: Underscore) => None , Some (ImportAlias :: Alias (alias)) => Some (alias) , None => Some (self . name (db)) , } } }
};
}
