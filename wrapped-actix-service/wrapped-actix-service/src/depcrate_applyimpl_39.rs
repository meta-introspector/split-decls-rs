// Generated macro for impl_39 (impl)
macro_rules! Depcrate_applyimpl_39 {
() => {
// Module: crate::apply
// Provides: {"impl_39"}
// Dependencies: {}
impl < SF , F , Fut , Req , In , Res , Err > ServiceFactory < Req > for ApplyFactory < SF , F , Req , In , Res , Err > where SF : ServiceFactory < In , Error = Err > , F : Fn (Req , & SF :: Service) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { type Response = Res ; type Error = Err ; type Config = SF :: Config ; type Service = Apply < SF :: Service , F , Req , In , Res , Err > ; type InitError = SF :: InitError ; type Future = ApplyServiceFactoryResponse < SF , F , Fut , Req , In , Res , Err > ; fn new_service (& self , cfg : SF :: Config) -> Self :: Future { let svc = self . factory . new_service (cfg) ; ApplyServiceFactoryResponse :: new (svc , self . wrap_fn . clone ()) } }
};
}
