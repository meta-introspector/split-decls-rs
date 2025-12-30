// Generated macro for impl_95 (impl)
macro_rules! Depcrate_fn_serviceimpl_95 {
() => {
// Module: crate::fn_service
// Provides: {"impl_95"}
// Dependencies: {}
impl < F , Fut , Req , Res , Err > Clone for FnService < F , Fut , Req , Res , Err > where F : FnMut (Req) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { fn clone (& self) -> Self { Self :: new (self . f . clone ()) } }
};
}
