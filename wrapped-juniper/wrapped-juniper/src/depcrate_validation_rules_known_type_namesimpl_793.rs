// Generated macro for impl_793 (impl)
macro_rules! Depcrate_validation_rules_known_type_namesimpl_793 {
() => {
// Module: crate::validation::rules::known_type_names
// Provides: {"impl_793"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for KnownTypeNames where S : ScalarValue , { fn enter_inline_fragment (& mut self , ctx : & mut ValidatorContext < 'a , S > , fragment : & 'a Spanning < InlineFragment < S > > ,) { if let Some (ref type_cond) = fragment . item . type_condition { validate_type (ctx , type_cond . item , & type_cond . span . start) ; } } fn enter_fragment_definition (& mut self , ctx : & mut ValidatorContext < 'a , S > , fragment : & 'a Spanning < Fragment < S > > ,) { let type_cond = & fragment . item . type_condition ; validate_type (ctx , type_cond . item , & type_cond . span . start) ; } fn enter_variable_definition (& mut self , ctx : & mut ValidatorContext < 'a , S > , (_ , var_def) : & 'a (Spanning < & 'a str > , VariableDefinition < S >) ,) { let type_name = var_def . var_type . item . innermost_name () ; validate_type (ctx , type_name , & var_def . var_type . span . start) ; } }
};
}
