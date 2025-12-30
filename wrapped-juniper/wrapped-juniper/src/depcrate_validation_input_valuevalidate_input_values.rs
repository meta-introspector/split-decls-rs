// Generated macro for validate_input_values (function)
macro_rules! Depcrate_validation_input_valuevalidate_input_values {
() => {
// Module: crate::validation::input_value
// Provides: {"validate_input_values"}
// Dependencies: {}
# [doc (hidden)] pub fn validate_input_values < S > (values : & Variables < S > , operation : & Spanning < Operation < S > > , schema : & SchemaType < S > ,) -> Vec < RuleError > where S : ScalarValue , { let mut errs = vec ! [] ; if let Some (ref vars) = operation . item . variables_definition { validate_var_defs (values , & vars . item , schema , & mut errs) ; } errs . sort () ; errs }
};
}
