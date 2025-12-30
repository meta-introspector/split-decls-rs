// Generated macro for impl_975 (impl)
macro_rules! Depcrate_machineimpl_975 {
() => {
// Module: crate::machine
// Provides: {"impl_975"}
// Dependencies: {}
impl < 'tcx > std :: fmt :: Debug for FrameExtra < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let FrameExtra { borrow_tracker , catch_unwind , timing : _ , user_relevance , data_race } = self ; f . debug_struct ("FrameData") . field ("borrow_tracker" , borrow_tracker) . field ("catch_unwind" , catch_unwind) . field ("user_relevance" , user_relevance) . field ("data_race" , data_race) . finish () } }
};
}
