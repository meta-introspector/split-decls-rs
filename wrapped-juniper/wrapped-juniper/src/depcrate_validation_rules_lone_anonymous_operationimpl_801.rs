// Generated macro for impl_801 (impl)
macro_rules! Depcrate_validation_rules_lone_anonymous_operationimpl_801 {
() => {
// Module: crate::validation::rules::lone_anonymous_operation
// Provides: {"impl_801"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for LoneAnonymousOperation where S : ScalarValue , { fn enter_document (& mut self , _ : & mut ValidatorContext < 'a , S > , doc : & 'a Document < S >) { self . operation_count = Some (doc . iter () . filter (| d | match * * d { Definition :: Operation (_) => true , Definition :: Fragment (_) => false , }) . count () ,) ; } fn enter_operation_definition (& mut self , ctx : & mut ValidatorContext < 'a , S > , op : & 'a Spanning < Operation < S > > ,) { if let Some (operation_count) = self . operation_count { if operation_count > 1 && op . item . name . is_none () { ctx . report_error (error_message () , & [op . span . start]) ; } } } }
};
}
