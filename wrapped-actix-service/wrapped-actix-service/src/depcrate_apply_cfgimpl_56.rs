// Generated macro for impl_56 (impl)
macro_rules! Depcrate_apply_cfgimpl_56 {
() => {
// Module: crate::apply_cfg
// Provides: {"impl_56"}
// Dependencies: {}
impl < SF , Req , F , Cfg , Fut , S > Clone for ApplyConfigServiceFactory < SF , Req , F , Cfg , Fut , S > where SF : ServiceFactory < Req , Config = () > , F : Fn (Cfg , & SF :: Service) -> Fut , Fut : Future < Output = Result < S , SF :: InitError > > , S : Service < Req > , { fn clone (& self) -> Self { Self { srv : self . srv . clone () , _phantom : PhantomData , } } }
};
}
