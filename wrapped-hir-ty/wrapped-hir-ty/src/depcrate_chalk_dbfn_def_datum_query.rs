// Generated macro for fn_def_datum_query (function)
macro_rules! Depcrate_chalk_dbfn_def_datum_query {
() => {
// Module: crate::chalk_db
// Provides: {"fn_def_datum_query"}
// Dependencies: {}
pub (crate) fn fn_def_datum_query (db : & dyn HirDatabase , callable_def : CallableDefId ,) -> Arc < FnDefDatum > { let generic_def = GenericDefId :: from_callable (db , callable_def) ; let generic_params = generics (db , generic_def) ; let (sig , binders) = db . callable_item_signature (callable_def) . into_value_and_skipped_binders () ; let bound_vars = generic_params . bound_vars_subst (db , DebruijnIndex :: INNERMOST) ; let where_clauses = convert_where_clauses (db , generic_def , & bound_vars) ; let bound = rust_ir :: FnDefDatumBound { inputs_and_output : chalk_ir :: Binders :: empty (Interner , rust_ir :: FnDefInputsAndOutputDatum { argument_types : sig . params () . to_vec () , return_type : sig . ret () . clone () , } . shifted_in (Interner) ,) , where_clauses , } ; let datum = FnDefDatum { id : callable_def . to_chalk (db) , sig : chalk_ir :: FnSig { abi : sig . abi , safety : chalk_ir :: Safety :: Safe , variadic : sig . is_varargs , } , binders : chalk_ir :: Binders :: new (binders , bound) , } ; Arc :: new (datum) }
};
}
