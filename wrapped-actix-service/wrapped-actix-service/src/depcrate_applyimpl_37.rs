// Generated macro for impl_37 (impl)
macro_rules! Depcrate_applyimpl_37 {
() => {
// Module: crate::apply
// Provides: {"impl_37"}
// Dependencies: {}
impl < SF , F , Fut , Req , In , Res , Err > ApplyFactory < SF , F , Req , In , Res , Err > where SF : ServiceFactory < In , Error = Err > , F : Fn (Req , & SF :: Service) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { # [doc = " Create new `ApplyFactory` new service instance"] fn new (factory : SF , wrap_fn : F) -> Self { Self { factory , wrap_fn , _phantom : PhantomData , } } }
};
}
