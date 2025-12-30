// Generated macro for FnServiceNoConfig (struct)
macro_rules! Depcrate_fn_serviceFnServiceNoConfig {
() => {
// Module: crate::fn_service
// Provides: {"FnServiceNoConfig"}
// Dependencies: {}
# [doc = " Converter for `Fn() -> Future<Service>` fn"] pub struct FnServiceNoConfig < F , Cfg , Srv , Req , Fut , Err > where F : Fn () -> Fut , Srv : Service < Req > , Fut : Future < Output = Result < Srv , Err > > , { f : F , _t : PhantomData < fn (Cfg , Req) > , }
};
}
