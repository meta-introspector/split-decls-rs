// Generated macro for impl_395 (impl)
macro_rules! Depcrateimpl_395 {
() => {
// Module: crate
// Provides: {"impl_395"}
// Dependencies: {}
impl TypeAlias { pub fn has_non_default_type_params (self , db : & dyn HirDatabase) -> bool { let subst = db . generic_defaults (self . id . into ()) ; (subst . is_empty () && db . generic_params (self . id . into ()) . len_type_or_consts () != 0) || subst . iter () . any (| ty | match ty . skip_binders () . data (Interner) { GenericArgData :: Ty (it) => it . is_unknown () , _ => false , }) } pub fn module (self , db : & dyn HirDatabase) -> Module { Module { id : self . id . module (db) } } pub fn ty (self , db : & dyn HirDatabase) -> Type < '_ > { Type :: from_def (db , self . id) } pub fn ty_placeholders (self , db : & dyn HirDatabase) -> Type < '_ > { Type :: from_def_placeholders (db , self . id) } pub fn name (self , db : & dyn HirDatabase) -> Name { db . type_alias_signature (self . id) . name . clone () } }
};
}
