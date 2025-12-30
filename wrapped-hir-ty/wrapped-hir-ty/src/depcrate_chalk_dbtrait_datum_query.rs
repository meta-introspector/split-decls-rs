// Generated macro for trait_datum_query (function)
macro_rules! Depcrate_chalk_dbtrait_datum_query {
() => {
// Module: crate::chalk_db
// Provides: {"trait_datum_query"}
// Dependencies: {}
pub (crate) fn trait_datum_query (db : & dyn HirDatabase , krate : Crate , trait_id : TraitId ,) -> Arc < TraitDatum > { debug ! ("trait_datum {:?}" , trait_id) ; let trait_ = from_chalk_trait_id (trait_id) ; let trait_data = db . trait_signature (trait_) ; debug ! ("trait {:?} = {:?}" , trait_id , trait_data . name) ; let generic_params = generics (db , trait_ . into ()) ; let bound_vars = generic_params . bound_vars_subst (db , DebruijnIndex :: INNERMOST) ; let flags = rust_ir :: TraitFlags { auto : trait_data . flags . contains (TraitFlags :: AUTO) , upstream : trait_ . lookup (db) . container . krate () != krate , non_enumerable : true , coinductive : false , marker : false , fundamental : trait_data . flags . contains (TraitFlags :: FUNDAMENTAL) , } ; let where_clauses = convert_where_clauses (db , trait_ . into () , & bound_vars) ; let associated_ty_ids = trait_ . trait_items (db) . associated_types () . map (to_assoc_type_id) . collect () ; let trait_datum_bound = rust_ir :: TraitDatumBound { where_clauses } ; let well_known = db . lang_attr (trait_ . into ()) . and_then (well_known_trait_from_lang_item) ; let trait_datum = TraitDatum { id : trait_id , binders : make_binders (db , & generic_params , trait_datum_bound) , flags , associated_ty_ids , well_known , } ; Arc :: new (trait_datum) }
};
}
