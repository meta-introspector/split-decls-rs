// Generated macro for impl_167 (impl)
macro_rules! Depcrate_map_errimpl_167 {
() => {
// Module: crate::map_err
// Provides: {"impl_167"}
// Dependencies: {}
impl < SF , Req , F , E > Future for MapErrServiceFuture < SF , Req , F , E > where SF : ServiceFactory < Req > , F : Fn (SF :: Error) -> E + Clone , { type Output = Result < MapErr < SF :: Service , Req , F , E > , SF :: InitError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Poll :: Ready (svc) = this . fut . poll (cx) ? { Poll :: Ready (Ok (MapErr :: new (svc , this . mapper . clone ()))) } else { Poll :: Pending } } }
};
}
