// Generated macro for impl_105 (impl)
macro_rules! Depcrate_fn_serviceimpl_105 {
() => {
// Module: crate::fn_service
// Provides: {"impl_105"}
// Dependencies: {}
impl < F , Fut , Cfg , Srv , Req , Err > FnServiceConfig < F , Fut , Cfg , Srv , Req , Err > where F : Fn (Cfg) -> Fut , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { fn new (f : F) -> Self { FnServiceConfig { f , _t : PhantomData } } }
};
}
