// Generated macro for impl_53 (impl)
macro_rules! Depcrate_apply_cfgimpl_53 {
() => {
// Module: crate::apply_cfg
// Provides: {"impl_53"}
// Dependencies: {}
impl < S1 , Req , F , Cfg , Fut , S2 , Err > Clone for ApplyConfigService < S1 , Req , F , Cfg , Fut , S2 , Err > where S1 : Service < Req > , F : Fn (Cfg , & S1) -> Fut , Fut : Future < Output = Result < S2 , Err > > , S2 : Service < Req > , { fn clone (& self) -> Self { ApplyConfigService { srv : self . srv . clone () , _phantom : PhantomData , } } }
};
}
