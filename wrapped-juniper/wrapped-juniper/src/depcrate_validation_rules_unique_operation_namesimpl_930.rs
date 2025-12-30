// Generated macro for impl_930 (impl)
macro_rules! Depcrate_validation_rules_unique_operation_namesimpl_930 {
() => {
// Module: crate::validation::rules::unique_operation_names
// Provides: {"impl_930"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for UniqueOperationNames < 'a > where S : ScalarValue , { fn enter_operation_definition (& mut self , ctx : & mut ValidatorContext < 'a , S > , op : & 'a Spanning < Operation < S > > ,) { if let Some (ref op_name) = op . item . name { match self . names . entry (op_name . item) { Entry :: Occupied (e) => { ctx . report_error (& error_message (op_name . item) , & [* e . get () , op . span . start]) ; } Entry :: Vacant (e) => { e . insert (op . span . start) ; } } } } }
};
}
