// Generated macro for is_absent_or_null (function)
macro_rules! Depcrate_validation_input_valueis_absent_or_null {
() => {
// Module: crate::validation::input_value
// Provides: {"is_absent_or_null"}
// Dependencies: {}
fn is_absent_or_null < S > (v : Option < & InputValue < S > >) -> bool where S : ScalarValue , { v . is_none_or (InputValue :: is_null) }
};
}
