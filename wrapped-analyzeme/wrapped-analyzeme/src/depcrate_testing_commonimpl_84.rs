// Generated macro for impl_84 (impl)
macro_rules! Depcrate_testing_commonimpl_84 {
() => {
// Module: crate::testing_common
// Provides: {"impl_84"}
// Dependencies: {}
impl ExpectedEvent { fn new (kind : & 'static str , label : & 'static str , args : & [& 'static str]) -> ExpectedEvent { ExpectedEvent { kind : Cow :: from (kind) , label : Cow :: from (label) , args : args . iter () . map (| & x | Cow :: from (x)) . collect () , } } }
};
}
