// Generated macro for impl_1100 (impl)
macro_rules! Depcrate_x509_verify_extension_policyimpl_1100 {
() => {
// Module: crate::x509::verify::extension_policy
// Provides: {"impl_1100"}
// Dependencies: {}
impl From < PyCriticality > for Criticality { fn from (criticality : PyCriticality) -> Criticality { match criticality { PyCriticality :: Critical => Criticality :: Critical , PyCriticality :: Agnostic => Criticality :: Agnostic , PyCriticality :: NonCritical => Criticality :: NonCritical , } } }
};
}
