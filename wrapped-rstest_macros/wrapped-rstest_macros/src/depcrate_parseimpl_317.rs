// Generated macro for impl_317 (impl)
macro_rules! Depcrate_parseimpl_317 {
() => {
// Module: crate::parse
// Provides: {"impl_317"}
// Dependencies: {}
impl CheckTimeoutAttributesFunction { pub (crate) fn take (self) -> Result < () , ErrorsVec > { self . 0 } fn check_if_can_implement_timeous (& self , timeouts : & [& syn :: Attribute] , asyncness : Option < & Async > ,) -> Option < syn :: Error > { if cfg ! (feature = "async-timeout") || timeouts . is_empty () { None } else { asyncness . map (| a | { syn :: Error :: new (a . span , "Enable async-timeout feature to use timeout in async tests" ,) }) } } }
};
}
