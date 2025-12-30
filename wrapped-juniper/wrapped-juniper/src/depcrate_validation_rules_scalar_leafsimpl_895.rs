// Generated macro for impl_895 (impl)
macro_rules! Depcrate_validation_rules_scalar_leafsimpl_895 {
() => {
// Module: crate::validation::rules::scalar_leafs
// Provides: {"impl_895"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for ScalarLeafs where S : ScalarValue , { fn enter_field (& mut self , ctx : & mut ValidatorContext < 'a , S > , field : & 'a Spanning < Field < S > >) { let field_name = & field . item . name . item ; let error = if let (Some (field_type) , Some (field_type_literal)) = (ctx . current_type () , ctx . current_type_literal ()) { match (field_type . is_leaf () , & field . item . selection_set) { (true , & Some (_)) => Some (RuleError :: new (& no_allowed_error_message (field_name , field_type_literal) , & [field . span . start] ,)) , (false , & None) => Some (RuleError :: new (& required_error_message (field_name , field_type_literal) , & [field . span . start] ,)) , _ => None , } } else { None } ; if let Some (error) = error { ctx . append_errors (vec ! [error]) ; } } }
};
}
