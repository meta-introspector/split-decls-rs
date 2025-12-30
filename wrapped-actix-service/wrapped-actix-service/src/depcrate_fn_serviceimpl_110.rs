// Generated macro for impl_110 (impl)
macro_rules! Depcrate_fn_serviceimpl_110 {
() => {
// Module: crate::fn_service
// Provides: {"impl_110"}
// Dependencies: {}
impl < F , Cfg , Srv , Req , Fut , Err > ServiceFactory < Req > for FnServiceNoConfig < F , Cfg , Srv , Req , Fut , Err > where F : Fn () -> Fut , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { type Response = Srv :: Response ; type Error = Srv :: Error ; type Config = Cfg ; type Service = Srv ; type InitError = Err ; type Future = Fut ; fn new_service (& self , _ : Cfg) -> Self :: Future { (self . f) () } }
};
}
