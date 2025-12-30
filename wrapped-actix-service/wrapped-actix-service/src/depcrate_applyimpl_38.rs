// Generated macro for impl_38 (impl)
macro_rules! Depcrate_applyimpl_38 {
() => {
// Module: crate::apply
// Provides: {"impl_38"}
// Dependencies: {}
impl < SF , F , Fut , Req , In , Res , Err > Clone for ApplyFactory < SF , F , Req , In , Res , Err > where SF : ServiceFactory < In , Error = Err > + Clone , F : Fn (Req , & SF :: Service) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { fn clone (& self) -> Self { Self { factory : self . factory . clone () , wrap_fn : self . wrap_fn . clone () , _phantom : PhantomData , } } }
};
}
