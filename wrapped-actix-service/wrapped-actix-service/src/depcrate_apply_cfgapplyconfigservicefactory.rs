// Generated macro for ApplyConfigServiceFactory (struct)
macro_rules! Depcrate_apply_cfgApplyConfigServiceFactory {
() => {
// Module: crate::apply_cfg
// Provides: {"ApplyConfigServiceFactory"}
// Dependencies: {}
# [doc = " Convert `Fn(&Config) -> Future<Service>` fn to NewService"] struct ApplyConfigServiceFactory < SF , Req , F , Cfg , Fut , S > where SF : ServiceFactory < Req , Config = () > , F : Fn (Cfg , & SF :: Service) -> Fut , Fut : Future < Output = Result < S , SF :: InitError > > , S : Service < Req > , { srv : Rc < (SF , F) > , _phantom : PhantomData < (Cfg , Req , Fut , S) > , }
};
}
