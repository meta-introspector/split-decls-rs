// Generated macro for impl_338 (impl)
macro_rules! Depcrate_validation_rules_scalar_leafsimpl_338 {
() => {
// Module: crate::validation::rules::scalar_leafs
// Provides: {"impl_338"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for ScalarLeafs { fn enter_field (& mut self , ctx : & mut VisitorContext < 'a > , field : & 'a Positioned < Field >) { if let Some (ty) = ctx . parent_type () { if let Some (schema_field) = ty . field_by_name (& field . node . name . node) { if let Some (ty) = ctx . registry . concrete_type_by_name (& schema_field . ty) { if ty . is_leaf () && ! field . node . selection_set . node . items . is_empty () { ctx . report_error (vec ! [field . pos] , format ! ("Field \"{}\" must not have a selection since type \"{}\" has no subfields" , field . node . name , ty . name ())) } else if ! ty . is_leaf () && field . node . selection_set . node . items . is_empty () { ctx . report_error (vec ! [field . pos] , format ! ("Field \"{}\" of type \"{}\" must have a selection of subfields" , field . node . name , ty . name ()) ,) } } } } } }
};
}
