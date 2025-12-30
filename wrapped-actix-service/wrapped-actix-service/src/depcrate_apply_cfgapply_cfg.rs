// Generated macro for apply_cfg (function)
macro_rules! Depcrate_apply_cfgapply_cfg {
() => {
// Module: crate::apply_cfg
// Provides: {"apply_cfg"}
// Dependencies: {}
# [doc = " Convert `Fn(Config, &Service1) -> Future<Service2>` fn to a service factory."] pub fn apply_cfg < S1 , Req , F , Cfg , Fut , S2 , Err > (srv : S1 , f : F ,) -> impl ServiceFactory < Req , Config = Cfg , Response = S2 :: Response , Error = S2 :: Error , Service = S2 , InitError = Err , Future = Fut , > + Clone where S1 : Service < Req > , F : Fn (Cfg , & S1) -> Fut , Fut : Future < Output = Result < S2 , Err > > , S2 : Service < Req > , { ApplyConfigService { srv : Rc :: new ((srv , f)) , _phantom : PhantomData , } }
};
}
