// Generated macro for impl_42 (impl)
macro_rules! Depcrate_applyimpl_42 {
() => {
// Module: crate::apply
// Provides: {"impl_42"}
// Dependencies: {}
impl < SF , F , Fut , Req , In , Res , Err > Future for ApplyServiceFactoryResponse < SF , F , Fut , Req , In , Res , Err > where SF : ServiceFactory < In , Error = Err > , F : Fn (Req , & SF :: Service) -> Fut , Fut : Future < Output = Result < Res , Err > > , { type Output = Result < Apply < SF :: Service , F , Req , In , Res , Err > , SF :: InitError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let svc = ready ! (this . fut . poll (cx)) ? ; Poll :: Ready (Ok (Apply :: new (svc , this . wrap_fn . take () . unwrap ()))) } }
};
}
