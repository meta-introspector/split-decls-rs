// Generated macro for impl_325 (impl)
macro_rules! Depcrate_fut_future_timeoutimpl_325 {
() => {
// Module: crate::fut::future::timeout
// Provides: {"impl_325"}
// Dependencies: {}
impl < F , A > ActorFuture < A > for Timeout < F > where F : ActorFuture < A > , A : Actor , { type Output = Result < F :: Output , () > ; fn poll (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { let this = self . project () ; match this . fut . poll (act , ctx , task) { Poll :: Ready (res) => Poll :: Ready (Ok (res)) , Poll :: Pending => this . timeout . poll (task) . map (Err) , } } }
};
}
