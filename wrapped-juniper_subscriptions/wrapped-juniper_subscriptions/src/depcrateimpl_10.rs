// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a , S > Stream for Connection < 'a , S > where S : ScalarValue + Send + Sync + 'a , { type Item = ExecutionOutput < S > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Option < Self :: Item > > { self . stream . as_mut () . poll_next (cx) } }
};
}
