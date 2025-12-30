// Generated macro for impl_112 (impl)
macro_rules! Depcrate_fn_serviceimpl_112 {
() => {
// Module: crate::fn_service
// Provides: {"impl_112"}
// Dependencies: {}
impl < F , Cfg , Srv , Req , Fut , Err > IntoServiceFactory < FnServiceNoConfig < F , Cfg , Srv , Req , Fut , Err > , Req > for F where F : Fn () -> Fut , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { fn into_factory (self) -> FnServiceNoConfig < F , Cfg , Srv , Req , Fut , Err > { FnServiceNoConfig :: new (self) } }
};
}
