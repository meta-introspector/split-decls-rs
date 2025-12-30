// Generated macro for impl_946 (impl)
macro_rules! Depcrate_validation_rules_variables_are_input_typesimpl_946 {
() => {
// Module: crate::validation::rules::variables_are_input_types
// Provides: {"impl_946"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for UniqueVariableNames where S : ScalarValue , { fn enter_variable_definition (& mut self , ctx : & mut ValidatorContext < 'a , S > , (var_name , var_def) : & 'a (Spanning < & 'a str > , VariableDefinition < S >) ,) { if let Some (var_type) = ctx . schema . concrete_type_by_name (var_def . var_type . item . innermost_name ()) { if ! var_type . is_input () { ctx . report_error (& error_message (var_name . item , & var_def . var_type . item) , & [var_def . var_type . span . start] ,) ; } } } }
};
}
