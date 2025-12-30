// Generated macro for impl_1859 (impl)
macro_rules! Depcrate_shims_envimpl_1859 {
() => {
// Module: crate::shims::env
// Provides: {"impl_1859"}
// Dependencies: {}
impl VisitProvenance for EnvVars < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self { EnvVars :: Uninit => { } EnvVars :: Unix (env) => env . visit_provenance (visit) , EnvVars :: Windows (env) => env . visit_provenance (visit) , } } }
};
}
