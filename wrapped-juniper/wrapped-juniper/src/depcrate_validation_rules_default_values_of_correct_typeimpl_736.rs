// Generated macro for impl_736 (impl)
macro_rules! Depcrate_validation_rules_default_values_of_correct_typeimpl_736 {
() => {
// Module: crate::validation::rules::default_values_of_correct_type
// Provides: {"impl_736"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for DefaultValuesOfCorrectType where S : ScalarValue , { fn enter_variable_definition (& mut self , ctx : & mut ValidatorContext < 'a , S > , (var_name , var_def) : & 'a (Spanning < & 'a str > , VariableDefinition < S >) ,) { if let Some (Spanning { item : ref var_value , ref span , }) = var_def . default_value { if var_def . var_type . item . is_non_null () { ctx . report_error (& non_null_error_message (var_name . item , & var_def . var_type . item) , & [span . start] ,) } else { let meta_type = ctx . schema . make_type (& var_def . var_type . item) ; if let Some (err) = validate_literal_value (ctx . schema , & meta_type , var_value) { ctx . report_error (& type_error_message (var_name . item , & var_def . var_type . item , err) , & [span . start] ,) ; } } } } }
};
}
