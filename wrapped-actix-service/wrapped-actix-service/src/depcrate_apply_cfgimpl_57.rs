// Generated macro for impl_57 (impl)
macro_rules! Depcrate_apply_cfgimpl_57 {
() => {
// Module: crate::apply_cfg
// Provides: {"impl_57"}
// Dependencies: {}
impl < SF , Req , F , Cfg , Fut , S > ServiceFactory < Req > for ApplyConfigServiceFactory < SF , Req , F , Cfg , Fut , S > where SF : ServiceFactory < Req , Config = () > , SF :: InitError : From < SF :: Error > , F : Fn (Cfg , & SF :: Service) -> Fut , Fut : Future < Output = Result < S , SF :: InitError > > , S : Service < Req > , { type Response = S :: Response ; type Error = S :: Error ; type Config = Cfg ; type Service = S ; type InitError = SF :: InitError ; type Future = ApplyConfigServiceFactoryResponse < SF , Req , F , Cfg , Fut , S > ; fn new_service (& self , cfg : Cfg) -> Self :: Future { ApplyConfigServiceFactoryResponse { cfg : Some (cfg) , store : self . srv . clone () , state : State :: A { fut : self . srv . 0 . new_service (()) , } , } } }
};
}
