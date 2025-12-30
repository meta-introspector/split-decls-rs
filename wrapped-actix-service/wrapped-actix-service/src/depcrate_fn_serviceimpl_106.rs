// Generated macro for impl_106 (impl)
macro_rules! Depcrate_fn_serviceimpl_106 {
() => {
// Module: crate::fn_service
// Provides: {"impl_106"}
// Dependencies: {}
impl < F , Fut , Cfg , Srv , Req , Err > Clone for FnServiceConfig < F , Fut , Cfg , Srv , Req , Err > where F : Fn (Cfg) -> Fut + Clone , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { fn clone (& self) -> Self { FnServiceConfig { f : self . f . clone () , _t : PhantomData , } } }
};
}
