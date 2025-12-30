// Generated macro for impl_886 (impl)
macro_rules! Depcrate_validation_rules_provided_non_null_argumentsimpl_886 {
() => {
// Module: crate::validation::rules::provided_non_null_arguments
// Provides: {"impl_886"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for ProvidedNonNullArguments where S : ScalarValue , { fn enter_field (& mut self , ctx : & mut ValidatorContext < 'a , S > , field : & 'a Spanning < Field < S > >) { let field_name = & field . item . name . item ; if let Some (& FieldType { arguments : Some (ref meta_args) , .. }) = ctx . parent_type () . and_then (| t | t . field_by_name (field_name)) { for meta_arg in meta_args { if meta_arg . arg_type . is_non_null () && meta_arg . default_value . is_none () && field . item . arguments . as_ref () . and_then (| args | args . item . get (& meta_arg . name)) . is_none () { ctx . report_error (& field_error_message (field_name , & meta_arg . name , & meta_arg . arg_type) , & [field . span . start] ,) ; } } } } fn enter_directive (& mut self , ctx : & mut ValidatorContext < 'a , S > , directive : & 'a Spanning < Directive < S > > ,) { let directive_name = & directive . item . name . item ; if let Some (DirectiveType { arguments : meta_args , .. }) = ctx . schema . directive_by_name (directive_name) { for meta_arg in meta_args { if meta_arg . arg_type . is_non_null () && directive . item . arguments . as_ref () . and_then (| args | args . item . get (& meta_arg . name)) . is_none () { ctx . report_error (& directive_error_message (directive_name , & meta_arg . name , & meta_arg . arg_type ,) , & [directive . span . start] ,) ; } } } } }
};
}
