// Generated macro for impl_107 (impl)
macro_rules! Depcrate_fn_serviceimpl_107 {
() => {
// Module: crate::fn_service
// Provides: {"impl_107"}
// Dependencies: {}
impl < F , Fut , Cfg , Srv , Req , Err > ServiceFactory < Req > for FnServiceConfig < F , Fut , Cfg , Srv , Req , Err > where F : Fn (Cfg) -> Fut , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { type Response = Srv :: Response ; type Error = Srv :: Error ; type Config = Cfg ; type Service = Srv ; type InitError = Err ; type Future = Fut ; fn new_service (& self , cfg : Cfg) -> Self :: Future { (self . f) (cfg) } }
};
}
