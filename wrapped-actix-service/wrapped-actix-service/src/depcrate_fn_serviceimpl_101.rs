// Generated macro for impl_101 (impl)
macro_rules! Depcrate_fn_serviceimpl_101 {
() => {
// Module: crate::fn_service
// Provides: {"impl_101"}
// Dependencies: {}
impl < F , Fut , Req , Res , Err > Service < Req > for FnServiceFactory < F , Fut , Req , Res , Err , () > where F : Fn (Req) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { type Response = Res ; type Error = Err ; type Future = Fut ; crate :: always_ready ! () ; fn call (& self , req : Req) -> Self :: Future { (self . f) (req) } }
};
}
