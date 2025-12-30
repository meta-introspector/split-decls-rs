// Generated macro for impl_109 (impl)
macro_rules! Depcrate_fn_serviceimpl_109 {
() => {
// Module: crate::fn_service
// Provides: {"impl_109"}
// Dependencies: {}
impl < F , Cfg , Srv , Req , Fut , Err > FnServiceNoConfig < F , Cfg , Srv , Req , Fut , Err > where F : Fn () -> Fut , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { fn new (f : F) -> Self { Self { f , _t : PhantomData } } }
};
}
