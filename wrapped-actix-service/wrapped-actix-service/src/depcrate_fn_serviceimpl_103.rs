// Generated macro for impl_103 (impl)
macro_rules! Depcrate_fn_serviceimpl_103 {
() => {
// Module: crate::fn_service
// Provides: {"impl_103"}
// Dependencies: {}
impl < F , Fut , Req , Res , Err , Cfg > IntoServiceFactory < FnServiceFactory < F , Fut , Req , Res , Err , Cfg > , Req > for F where F : Fn (Req) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { fn into_factory (self) -> FnServiceFactory < F , Fut , Req , Res , Err , Cfg > { FnServiceFactory :: new (self) } }
};
}
