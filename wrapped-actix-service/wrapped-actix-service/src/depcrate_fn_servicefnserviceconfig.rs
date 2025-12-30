// Generated macro for FnServiceConfig (struct)
macro_rules! Depcrate_fn_serviceFnServiceConfig {
() => {
// Module: crate::fn_service
// Provides: {"FnServiceConfig"}
// Dependencies: {}
# [doc = " Convert `Fn(&Config) -> Future<Service>` fn to NewService"] pub struct FnServiceConfig < F , Fut , Cfg , Srv , Req , Err > where F : Fn (Cfg) -> Fut , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { f : F , _t : PhantomData < fn (Cfg , Req) -> (Fut , Srv , Err) > , }
};
}
