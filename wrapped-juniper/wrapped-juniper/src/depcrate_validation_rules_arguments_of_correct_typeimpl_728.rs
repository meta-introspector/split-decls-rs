// Generated macro for impl_728 (impl)
macro_rules! Depcrate_validation_rules_arguments_of_correct_typeimpl_728 {
() => {
// Module: crate::validation::rules::arguments_of_correct_type
// Provides: {"impl_728"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for ArgumentsOfCorrectType < 'a , S > where S : ScalarValue , { fn enter_directive (& mut self , ctx : & mut ValidatorContext < 'a , S > , directive : & 'a Spanning < Directive < S > > ,) { self . current_args = ctx . schema . directive_by_name (directive . item . name . item) . map (| d | & d . arguments) ; } fn exit_directive (& mut self , _ : & mut ValidatorContext < 'a , S > , _ : & 'a Spanning < Directive < S > >) { self . current_args = None ; } fn enter_field (& mut self , ctx : & mut ValidatorContext < 'a , S > , field : & 'a Spanning < Field < S > >) { self . current_args = ctx . parent_type () . and_then (| t | t . field_by_name (field . item . name . item)) . and_then (| f | f . arguments . as_ref ()) ; } fn exit_field (& mut self , _ : & mut ValidatorContext < 'a , S > , _ : & 'a Spanning < Field < S > >) { self . current_args = None ; } fn enter_argument (& mut self , ctx : & mut ValidatorContext < 'a , S > , (arg_name , arg_value) : & 'a (Spanning < & 'a str > , Spanning < InputValue < S > >) ,) { if let Some (argument_meta) = self . current_args . and_then (| args | args . iter () . find (| a | a . name == arg_name . item)) { let meta_type = ctx . schema . make_type (& argument_meta . arg_type) ; if let Some (err) = validate_literal_value (ctx . schema , & meta_type , & arg_value . item) { ctx . report_error (& error_message (arg_name . item , err) , & [arg_value . span . start]) ; } } } }
};
}
