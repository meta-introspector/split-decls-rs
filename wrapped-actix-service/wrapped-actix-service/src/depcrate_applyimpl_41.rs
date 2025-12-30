// Generated macro for impl_41 (impl)
macro_rules! Depcrate_applyimpl_41 {
() => {
// Module: crate::apply
// Provides: {"impl_41"}
// Dependencies: {}
impl < SF , F , Fut , Req , In , Res , Err > ApplyServiceFactoryResponse < SF , F , Fut , Req , In , Res , Err > where SF : ServiceFactory < In , Error = Err > , F : Fn (Req , & SF :: Service) -> Fut , Fut : Future < Output = Result < Res , Err > > , { fn new (fut : SF :: Future , wrap_fn : F) -> Self { Self { fut , wrap_fn : Some (wrap_fn) , _phantom : PhantomData , } } }
};
}
