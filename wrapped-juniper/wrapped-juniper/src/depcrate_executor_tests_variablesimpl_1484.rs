// Generated macro for impl_1484 (impl)
macro_rules! Depcrate_executor_tests_variablesimpl_1484 {
() => {
// Module: crate::executor_tests::variables
// Provides: {"impl_1484"}
// Dependencies: {}
impl TestComplexScalar { fn to_output (& self) -> & 'static str { "SerializedValue" } fn from_input (s : & str) -> Result < Self , Box < str > > { if s == "SerializedValue" { Ok (Self) } else { Err (format ! (r#"Expected "SerializedValue" string, found: "{s}""#) . into ()) } } }
};
}
