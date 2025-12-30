// Generated macro for impl_102 (impl)
macro_rules! Depcrate_fn_serviceimpl_102 {
() => {
// Module: crate::fn_service
// Provides: {"impl_102"}
// Dependencies: {}
impl < F , Fut , Req , Res , Err , Cfg > ServiceFactory < Req > for FnServiceFactory < F , Fut , Req , Res , Err , Cfg > where F : Fn (Req) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { type Response = Res ; type Error = Err ; type Config = Cfg ; type Service = FnService < F , Fut , Req , Res , Err > ; type InitError = () ; type Future = Ready < Result < Self :: Service , Self :: InitError > > ; fn new_service (& self , _ : Cfg) -> Self :: Future { ok (FnService :: new (self . f . clone ())) } }
};
}
