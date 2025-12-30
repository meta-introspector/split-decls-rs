// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl Future for Timer { type Output = Instant ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . poll_next (cx) { Poll :: Ready (Some (when)) => Poll :: Ready (when) , Poll :: Pending => Poll :: Pending , Poll :: Ready (None) => unreachable ! () , } } }
};
}
