// Generated macro for impl_360 (impl)
macro_rules! Depcrate_validation_rules_variables_are_input_typesimpl_360 {
() => {
// Module: crate::validation::rules::variables_are_input_types
// Provides: {"impl_360"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for VariablesAreInputTypes { fn enter_variable_definition (& mut self , ctx : & mut VisitorContext < 'a > , variable_definition : & 'a Positioned < VariableDefinition > ,) { if let Some (ty) = ctx . registry . concrete_type_by_parsed_type (& variable_definition . node . var_type . node) { if ! ty . is_input () { ctx . report_error (vec ! [variable_definition . pos] , format ! ("Variable \"{}\" cannot be of non-input type \"{}\"" , variable_definition . node . name . node , ty . name ()) ,) ; } } } }
};
}
