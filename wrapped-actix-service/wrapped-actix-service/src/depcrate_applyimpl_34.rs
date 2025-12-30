// Generated macro for impl_34 (impl)
macro_rules! Depcrate_applyimpl_34 {
() => {
// Module: crate::apply
// Provides: {"impl_34"}
// Dependencies: {}
impl < S , F , Fut , Req , In , Res , Err > Clone for Apply < S , F , Req , In , Res , Err > where S : Service < In , Error = Err > + Clone , F : Fn (Req , & S) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { fn clone (& self) -> Self { Apply { service : self . service . clone () , wrap_fn : self . wrap_fn . clone () , _phantom : PhantomData , } } }
};
}
