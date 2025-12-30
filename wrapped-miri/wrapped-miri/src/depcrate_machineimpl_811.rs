// Generated macro for impl_811 (impl)
macro_rules! Depcrate_machineimpl_811 {
() => {
// Module: crate::machine
// Provides: {"impl_811"}
// Dependencies: {}
impl VisitProvenance for FrameExtra < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let FrameExtra { catch_unwind , borrow_tracker , timing : _ , is_user_relevant : _ , data_race : _ , } = self ; catch_unwind . visit_provenance (visit) ; borrow_tracker . visit_provenance (visit) ; } }
};
}
