// Generated macro for service (function)
macro_rules! Depcrate_boxedservice {
() => {
// Module: crate::boxed
// Provides: {"service"}
// Dependencies: {}
# [doc = " Wraps service as a trait object using [`BoxService`]."] pub fn service < S , Req > (service : S) -> BoxService < Req , S :: Response , S :: Error > where S : Service < Req > + 'static , Req : 'static , S :: Future : 'static , { Box :: new (ServiceWrapper :: new (service)) }
};
}
