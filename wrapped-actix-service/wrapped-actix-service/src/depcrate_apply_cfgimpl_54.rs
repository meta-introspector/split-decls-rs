// Generated macro for impl_54 (impl)
macro_rules! Depcrate_apply_cfgimpl_54 {
() => {
// Module: crate::apply_cfg
// Provides: {"impl_54"}
// Dependencies: {}
impl < S1 , Req , F , Cfg , Fut , S2 , Err > ServiceFactory < Req > for ApplyConfigService < S1 , Req , F , Cfg , Fut , S2 , Err > where S1 : Service < Req > , F : Fn (Cfg , & S1) -> Fut , Fut : Future < Output = Result < S2 , Err > > , S2 : Service < Req > , { type Response = S2 :: Response ; type Error = S2 :: Error ; type Config = Cfg ; type Service = S2 ; type InitError = Err ; type Future = Fut ; fn new_service (& self , cfg : Cfg) -> Self :: Future { let (t , f) = & * self . srv ; f (cfg , t) } }
};
}
