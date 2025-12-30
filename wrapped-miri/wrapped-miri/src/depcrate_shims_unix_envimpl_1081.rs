// Generated macro for impl_1081 (impl)
macro_rules! Depcrate_shims_unix_envimpl_1081 {
() => {
// Module: crate::shims::unix::env
// Provides: {"impl_1081"}
// Dependencies: {}
impl VisitProvenance for UnixEnvVars < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let UnixEnvVars { map , environ } = self ; environ . visit_provenance (visit) ; for ptr in map . values () { ptr . visit_provenance (visit) ; } } }
};
}
