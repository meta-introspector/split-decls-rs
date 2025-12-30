// Generated macro for impl_100 (impl)
macro_rules! Depcrate_fn_serviceimpl_100 {
() => {
// Module: crate::fn_service
// Provides: {"impl_100"}
// Dependencies: {}
impl < F , Fut , Req , Res , Err , Cfg > Clone for FnServiceFactory < F , Fut , Req , Res , Err , Cfg > where F : Fn (Req) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { fn clone (& self) -> Self { Self :: new (self . f . clone ()) } }
};
}
