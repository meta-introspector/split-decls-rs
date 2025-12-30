// Generated macro for impl_1769 (impl)
macro_rules! Depcrate_shims_tlsimpl_1769 {
() => {
// Module: crate::shims::tls
// Provides: {"impl_1769"}
// Dependencies: {}
impl VisitProvenance for TlsData < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let TlsData { keys , macos_thread_dtors , next_key : _ } = self ; for scalar in keys . values () . flat_map (| v | v . data . values ()) { scalar . visit_provenance (visit) ; } for (_ , scalar) in macos_thread_dtors . values () . flatten () { scalar . visit_provenance (visit) ; } } }
};
}
