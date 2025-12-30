// Generated macro for impl_233 (impl)
macro_rules! Depcrate_check_wfcheckimpl_233 {
() => {
// Module: crate::check::wfcheck
// Provides: {"impl_233"}
// Dependencies: {}
impl < 'a , 'tcx > Deref for WfCheckingCtxt < 'a , 'tcx > { type Target = ObligationCtxt < 'a , 'tcx , FulfillmentError < 'tcx > > ; fn deref (& self) -> & Self :: Target { & self . ocx } }
};
}
