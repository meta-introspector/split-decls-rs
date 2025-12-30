// Generated macro for fn_service (function)
macro_rules! Depcrate_fn_servicefn_service {
() => {
// Module: crate::fn_service
// Provides: {"fn_service"}
// Dependencies: {}
# [doc = " Create `ServiceFactory` for function that can act as a `Service`"] pub fn fn_service < F , Fut , Req , Res , Err , Cfg > (f : F) -> FnServiceFactory < F , Fut , Req , Res , Err , Cfg > where F : Fn (Req) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { FnServiceFactory :: new (f) }
};
}
