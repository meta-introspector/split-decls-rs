// Generated macro for apply_cfg_factory (function)
macro_rules! Depcrate_apply_cfgapply_cfg_factory {
() => {
// Module: crate::apply_cfg
// Provides: {"apply_cfg_factory"}
// Dependencies: {}
# [doc = " Convert `Fn(Config, &ServiceFactory1) -> Future<ServiceFactory2>` fn to a service factory."] # [doc = ""] # [doc = " Service1 get constructed from `T` factory."] pub fn apply_cfg_factory < SF , Req , F , Cfg , Fut , S > (factory : SF , f : F ,) -> impl ServiceFactory < Req , Config = Cfg , Response = S :: Response , Error = S :: Error , Service = S , InitError = SF :: InitError , > + Clone where SF : ServiceFactory < Req , Config = () > , F : Fn (Cfg , & SF :: Service) -> Fut , SF :: InitError : From < SF :: Error > , Fut : Future < Output = Result < S , SF :: InitError > > , S : Service < Req > , { ApplyConfigServiceFactory { srv : Rc :: new ((factory , f)) , _phantom : PhantomData , } }
};
}
