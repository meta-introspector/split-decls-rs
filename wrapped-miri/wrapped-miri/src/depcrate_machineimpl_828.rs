// Generated macro for impl_828 (impl)
macro_rules! Depcrate_machineimpl_828 {
() => {
// Module: crate::machine
// Provides: {"impl_828"}
// Dependencies: {}
impl VisitProvenance for AllocExtra < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let AllocExtra { borrow_tracker , data_race , backtrace : _ , sync : _ } = self ; borrow_tracker . visit_provenance (visit) ; data_race . visit_provenance (visit) ; } }
};
}
