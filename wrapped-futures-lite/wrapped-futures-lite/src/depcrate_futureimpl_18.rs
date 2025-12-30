// Generated macro for impl_18 (impl)
macro_rules! Depcrate_futureimpl_18 {
() => {
// Module: crate::future
// Provides: {"impl_18"}
// Dependencies: {}
impl < T , F > Future for PollOnce < F > where F : Future < Output = T > , { type Output = Option < T > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . f . poll (cx) { Poll :: Ready (t) => Poll :: Ready (Some (t)) , Poll :: Pending => Poll :: Ready (None) , } } }
};
}
