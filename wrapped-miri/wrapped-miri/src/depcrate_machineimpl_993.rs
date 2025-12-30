// Generated macro for impl_993 (impl)
macro_rules! Depcrate_machineimpl_993 {
() => {
// Module: crate::machine
// Provides: {"impl_993"}
// Dependencies: {}
impl VisitProvenance for AllocExtra < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let AllocExtra { borrow_tracker , data_race , backtrace : _ , sync_objs : _ } = self ; borrow_tracker . visit_provenance (visit) ; data_race . visit_provenance (visit) ; } }
};
}
