// Generated macro for impl_1310 (impl)
macro_rules! Depcrate_shims_unix_fsimpl_1310 {
() => {
// Module: crate::shims::unix::fs
// Provides: {"impl_1310"}
// Dependencies: {}
impl VisitProvenance for DirTable { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let DirTable { streams , next_id : _ } = self ; for dir in streams . values () { dir . entry . visit_provenance (visit) ; } } }
};
}
