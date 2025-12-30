// Generated macro for impl_611 (impl)
macro_rules! Depcrate_service_oneshotimpl_611 {
() => {
// Module: crate::service::oneshot
// Provides: {"impl_611"}
// Dependencies: {}
impl < S , Req > Oneshot < S , Req > where S : Service < Req > , { pub (crate) const fn new (svc : S , req : Req) -> Self { Oneshot :: NotReady { svc , req : Some (req) , } } }
};
}
