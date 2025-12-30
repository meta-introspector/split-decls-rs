// Generated macro for impl_888 (impl)
macro_rules! Depcrate_provenance_gcimpl_888 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_888"}
// Dependencies: {}
impl VisitProvenance for IoError { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { use crate :: shims :: io_error :: IoError :: * ; match self { LibcError (_name) => () , WindowsError (_name) => () , HostError (_io_error) => () , Raw (scalar) => scalar . visit_provenance (visit) , } } }
};
}
