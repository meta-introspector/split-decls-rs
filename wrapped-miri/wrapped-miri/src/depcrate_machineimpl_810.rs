// Generated macro for impl_810 (impl)
macro_rules! Depcrate_machineimpl_810 {
() => {
// Module: crate::machine
// Provides: {"impl_810"}
// Dependencies: {}
impl < 'tcx > std :: fmt :: Debug for FrameExtra < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let FrameExtra { borrow_tracker , catch_unwind , timing : _ , is_user_relevant , data_race } = self ; f . debug_struct ("FrameData") . field ("borrow_tracker" , borrow_tracker) . field ("catch_unwind" , catch_unwind) . field ("is_user_relevant" , is_user_relevant) . field ("data_race" , data_race) . finish () } }
};
}
