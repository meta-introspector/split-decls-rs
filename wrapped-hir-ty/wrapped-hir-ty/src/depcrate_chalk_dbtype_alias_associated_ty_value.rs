// Generated macro for type_alias_associated_ty_value (function)
macro_rules! Depcrate_chalk_dbtype_alias_associated_ty_value {
() => {
// Module: crate::chalk_db
// Provides: {"type_alias_associated_ty_value"}
// Dependencies: {}
fn type_alias_associated_ty_value (db : & dyn HirDatabase , _krate : Crate , type_alias : TypeAliasId ,) -> Arc < AssociatedTyValue > { let type_alias_data = db . type_alias_signature (type_alias) ; let impl_id = match type_alias . lookup (db) . container { ItemContainerId :: ImplId (it) => it , _ => panic ! ("assoc ty value should be in impl") , } ; let trait_ref = db . impl_trait (impl_id) . expect ("assoc ty value should not exist") . into_value_and_skipped_binders () . 0 ; let assoc_ty = trait_ref . hir_trait_id () . trait_items (db) . associated_type_by_name (& type_alias_data . name) . expect ("assoc ty value should not exist") ; let (ty , binders) = db . ty (type_alias . into ()) . into_value_and_skipped_binders () ; let value_bound = rust_ir :: AssociatedTyValueBound { ty } ; let value = rust_ir :: AssociatedTyValue { impl_id : impl_id . to_chalk (db) , associated_ty_id : to_assoc_type_id (assoc_ty) , value : chalk_ir :: Binders :: new (binders , value_bound) , } ; Arc :: new (value) }
};
}
