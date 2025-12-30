// Generated macro for rc_service (function)
macro_rules! Depcrate_boxedrc_service {
() => {
// Module: crate::boxed
// Provides: {"rc_service"}
// Dependencies: {}
# [doc = " Wraps service as a trait object using [`RcService`]."] pub fn rc_service < S , Req > (service : S) -> RcService < Req , S :: Response , S :: Error > where S : Service < Req > + 'static , Req : 'static , S :: Future : 'static , { Rc :: new (ServiceWrapper :: new (service)) }
};
}
