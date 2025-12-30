// Generated macro for impl_60 (impl)
macro_rules! Depcrate_apply_cfgimpl_60 {
() => {
// Module: crate::apply_cfg
// Provides: {"impl_60"}
// Dependencies: {}
impl < SF , Req , F , Cfg , Fut , S > Future for ApplyConfigServiceFactoryResponse < SF , Req , F , Cfg , Fut , S > where SF : ServiceFactory < Req , Config = () > , SF :: InitError : From < SF :: Error > , F : Fn (Cfg , & SF :: Service) -> Fut , Fut : Future < Output = Result < S , SF :: InitError > > , S : Service < Req > , { type Output = Result < S , SF :: InitError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . as_mut () . project () ; match this . state . as_mut () . project () { StateProj :: A { fut } => { let svc = ready ! (fut . poll (cx)) ? ; this . state . set (State :: B { svc }) ; self . poll (cx) } StateProj :: B { svc } => { ready ! (svc . poll_ready (cx)) ? ; { let (_ , f) = & * * this . store ; let fut = f (this . cfg . take () . unwrap () , svc) ; this . state . set (State :: C { fut }) ; } self . poll (cx) } StateProj :: C { fut } => fut . poll (cx) , } } }
};
}
