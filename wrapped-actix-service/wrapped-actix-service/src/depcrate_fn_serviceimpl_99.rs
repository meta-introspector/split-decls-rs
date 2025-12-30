// Generated macro for impl_99 (impl)
macro_rules! Depcrate_fn_serviceimpl_99 {
() => {
// Module: crate::fn_service
// Provides: {"impl_99"}
// Dependencies: {}
impl < F , Fut , Req , Res , Err , Cfg > FnServiceFactory < F , Fut , Req , Res , Err , Cfg > where F : Fn (Req) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { fn new (f : F) -> Self { FnServiceFactory { f , _t : PhantomData } } }
};
}
