// Generated macro for impl_938 (impl)
macro_rules! Depcrate_validation_rules_unique_variable_namesimpl_938 {
() => {
// Module: crate::validation::rules::unique_variable_names
// Provides: {"impl_938"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for UniqueVariableNames < 'a > where S : ScalarValue , { fn enter_operation_definition (& mut self , _ : & mut ValidatorContext < 'a , S > , _ : & 'a Spanning < Operation < S > > ,) { self . names = HashMap :: new () ; } fn enter_variable_definition (& mut self , ctx : & mut ValidatorContext < 'a , S > , (var_name , _) : & 'a (Spanning < & 'a str > , VariableDefinition < S >) ,) { match self . names . entry (var_name . item) { Entry :: Occupied (e) => { ctx . report_error (& error_message (var_name . item) , & [* e . get () , var_name . span . start] ,) ; } Entry :: Vacant (e) => { e . insert (var_name . span . start) ; } } } }
};
}
