// Generated macro for impl_1591 (impl)
macro_rules! Depcrate_disallowed_script_identsimpl_1591 {
() => {
// Module: crate::disallowed_script_idents
// Provides: {"impl_1591"}
// Dependencies: {}
impl DisallowedScriptIdents { pub fn new (conf : & 'static Conf) -> Self { Self { whitelist : conf . allowed_scripts . iter () . map (String :: as_str) . filter_map (Script :: from_full_name) . collect () , } } }
};
}
