// Generated macro for impl_1043 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1043 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1043"}
// Dependencies: {}
impl < S > FromInputValue < S > for ComplexInput where S : ScalarValue , { type Error = FieldError < S > ; fn from_input_value (v : & InputValue < S >) -> Result < ComplexInput , Self :: Error > { let obj = v . to_object_value () . ok_or ("Expected object") ? ; Ok (ComplexInput { required_field : obj . get ("requiredField") . map (| v | v . convert ()) . transpose () ? . ok_or ("Expected requiredField") ? , int_field : obj . get ("intField") . map (| v | v . convert ()) . transpose () ? . ok_or ("Expected intField") ? , string_field : obj . get ("stringField") . map (| v | v . convert ()) . transpose () ? . ok_or ("Expected stringField") ? , boolean_field : obj . get ("booleanField") . map (| v | v . convert ()) . transpose () ? . ok_or ("Expected booleanField") ? , string_list_field : obj . get ("stringListField") . map (| v | v . convert () . map_err (IntoFieldError :: into_field_error)) . transpose () ? . ok_or ("Expected stringListField") ? , }) } }
};
}
