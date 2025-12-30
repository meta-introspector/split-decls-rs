// Generated macro for ApplyConfigService (struct)
macro_rules! Depcrate_apply_cfgApplyConfigService {
() => {
// Module: crate::apply_cfg
// Provides: {"ApplyConfigService"}
// Dependencies: {}
# [doc = " Convert `Fn(Config, &Server) -> Future<Service>` fn to NewService\\"] struct ApplyConfigService < S1 , Req , F , Cfg , Fut , S2 , Err > where S1 : Service < Req > , F : Fn (Cfg , & S1) -> Fut , Fut : Future < Output = Result < S2 , Err > > , S2 : Service < Req > , { srv : Rc < (S1 , F) > , _phantom : PhantomData < (Cfg , Req , Fut , S2) > , }
};
}
