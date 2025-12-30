// Generated macro for impl_128 (impl)
macro_rules! Depcrate_mapimpl_128 {
() => {
// Module: crate::map
// Provides: {"impl_128"}
// Dependencies: {}
impl < A , F , Req , Res > Future for MapFuture < A , F , Req , Res > where A : Service < Req > , F : FnMut (A :: Response) -> Res , { type Output = Result < Res , A :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; match this . fut . poll (cx) { Poll :: Ready (Ok (resp)) => Poll :: Ready (Ok ((this . f) (resp))) , Poll :: Ready (Err (err)) => Poll :: Ready (Err (err)) , Poll :: Pending => Poll :: Pending , } } }
};
}
