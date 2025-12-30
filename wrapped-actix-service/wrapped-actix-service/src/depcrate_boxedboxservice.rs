// Generated macro for BoxService (type)
macro_rules! Depcrate_boxedBoxService {
() => {
// Module: crate::boxed
// Provides: {"BoxService"}
// Dependencies: {}
# [doc = " Type alias for service trait object using [`Box`]."] pub type BoxService < Req , Res , Err > = Box < dyn Service < Req , Response = Res , Error = Err , Future = BoxFuture < Result < Res , Err > > > > ;
};
}
