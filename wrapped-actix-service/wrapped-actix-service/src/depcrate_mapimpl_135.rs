// Generated macro for impl_135 (impl)
macro_rules! Depcrate_mapimpl_135 {
() => {
// Module: crate::map
// Provides: {"impl_135"}
// Dependencies: {}
impl < A , F , Req , Res > Future for MapServiceFuture < A , F , Req , Res > where A : ServiceFactory < Req > , F : FnMut (A :: Response) -> Res , { type Output = Result < Map < A :: Service , F , Req , Res > , A :: InitError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Poll :: Ready (svc) = this . fut . poll (cx) ? { Poll :: Ready (Ok (Map :: new (svc , this . f . take () . unwrap ()))) } else { Poll :: Pending } } }
};
}
