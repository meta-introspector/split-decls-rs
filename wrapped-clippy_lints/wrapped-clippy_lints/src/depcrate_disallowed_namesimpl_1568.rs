// Generated macro for impl_1568 (impl)
macro_rules! Depcrate_disallowed_namesimpl_1568 {
() => {
// Module: crate::disallowed_names
// Provides: {"impl_1568"}
// Dependencies: {}
impl DisallowedNames { pub fn new (conf : & 'static Conf) -> Self { Self { disallow : conf . disallowed_names . iter () . map (| x | Symbol :: intern (x)) . collect () , } } }
};
}
