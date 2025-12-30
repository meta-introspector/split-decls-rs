// Generated macro for impl_315 (impl)
macro_rules! Depcrate_concurrent_stream_limitimpl_315 {
() => {
// Module: crate::concurrent_stream::limit
// Provides: {"impl_315"}
// Dependencies: {}
impl < C , Item , Fut > Consumer < Item , Fut > for LimitConsumer < C > where Fut : Future < Output = Item > , C : Consumer < Item , Fut > , { type Output = C :: Output ; async fn send (self : Pin < & mut Self > , future : Fut) -> super :: ConsumerState { let this = self . project () ; this . inner . send (future) . await } async fn progress (self : Pin < & mut Self >) -> super :: ConsumerState { let this = self . project () ; this . inner . progress () . await } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let this = self . project () ; this . inner . flush () . await } }
};
}
