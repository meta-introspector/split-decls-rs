// Generated macro for associated_ty_value_query (function)
macro_rules! Depcrate_chalk_dbassociated_ty_value_query {
() => {
// Module: crate::chalk_db
// Provides: {"associated_ty_value_query"}
// Dependencies: {}
pub (crate) fn associated_ty_value_query (db : & dyn HirDatabase , krate : Crate , id : AssociatedTyValueId ,) -> Arc < AssociatedTyValue > { let type_alias : TypeAliasAsValue = from_chalk (db , id) ; type_alias_associated_ty_value (db , krate , type_alias . 0) }
};
}
