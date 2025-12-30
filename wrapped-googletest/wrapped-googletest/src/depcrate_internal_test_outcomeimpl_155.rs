// Generated macro for impl_155 (impl)
macro_rules! Depcrate_internal_test_outcomeimpl_155 {
() => {
// Module: crate::internal::test_outcome
// Provides: {"impl_155"}
// Dependencies: {}
impl Display for TestAssertionFailure { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { writeln ! (f , "{}" , self . description) ? ; if let Some (custom_message) = & self . custom_message { writeln ! (f , "{custom_message}") ? ; } writeln ! (f , "  at {}" , self . location) } }
};
}
