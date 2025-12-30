// Generated macro for impl_153 (impl)
macro_rules! Depcrate_internal_test_outcomeimpl_153 {
() => {
// Module: crate::internal::test_outcome
// Provides: {"impl_153"}
// Dependencies: {}
impl Display for Location { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { match self { Location :: Real (l) => write ! (f , "{l}") , Location :: Fake { file , line , column } => write ! (f , "{file}:{line}:{column}") , } } }
};
}
