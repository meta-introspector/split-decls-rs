// Generated macro for impl_351 (impl)
macro_rules! Depcrate_validation_rules_unique_variable_namesimpl_351 {
() => {
// Module: crate::validation::rules::unique_variable_names
// Provides: {"impl_351"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for UniqueVariableNames < 'a > { fn enter_operation_definition (& mut self , _ctx : & mut VisitorContext < 'a > , _name : Option < & 'a Name > , _operation_definition : & 'a Positioned < OperationDefinition > ,) { self . names . clear () ; } fn enter_variable_definition (& mut self , ctx : & mut VisitorContext < 'a > , variable_definition : & 'a Positioned < VariableDefinition > ,) { if ! self . names . insert (& variable_definition . node . name . node) { ctx . report_error (vec ! [variable_definition . pos] , format ! ("There can only be one variable named \"${}\"" , variable_definition . node . name . node) ,) ; } } }
};
}
