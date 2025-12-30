// Generated macro for impl_744 (impl)
macro_rules! Depcrate_validation_rules_disable_introspectionimpl_744 {
() => {
// Module: crate::validation::rules::disable_introspection
// Provides: {"impl_744"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for DisableIntrospection where S : ScalarValue , { fn enter_field (& mut self , context : & mut ValidatorContext < 'a , S > , field : & 'a Spanning < Field < S > > ,) { let field_name = field . item . name . item ; if matches ! (field_name , "__schema" | "__type") { context . report_error (& error_message (field_name) , & [field . item . name . span . start]) ; } } }
};
}
