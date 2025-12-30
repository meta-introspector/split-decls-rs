// Generated macro for impl_111 (impl)
macro_rules! Depcrate_fn_serviceimpl_111 {
() => {
// Module: crate::fn_service
// Provides: {"impl_111"}
// Dependencies: {}
impl < F , Cfg , Srv , Req , Fut , Err > Clone for FnServiceNoConfig < F , Cfg , Srv , Req , Fut , Err > where F : Fn () -> Fut + Clone , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { fn clone (& self) -> Self { Self :: new (self . f . clone ()) } }
};
}
