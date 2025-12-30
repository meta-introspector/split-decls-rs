// Generated macro for RcService (type)
macro_rules! Depcrate_boxedRcService {
() => {
// Module: crate::boxed
// Provides: {"RcService"}
// Dependencies: {}
# [doc = " Type alias for service trait object using [`Rc`]."] pub type RcService < Req , Res , Err > = Rc < dyn Service < Req , Response = Res , Error = Err , Future = BoxFuture < Result < Res , Err > > > > ;
};
}
