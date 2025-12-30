// Generated macro for impl_339 (impl)
macro_rules! Depcrate_concurrent_stream_takeimpl_339 {
() => {
// Module: crate::concurrent_stream::take
// Provides: {"impl_339"}
// Dependencies: {}
impl < C , Item , Fut > Consumer < Item , Fut > for TakeConsumer < C > where Fut : Future < Output = Item > , C : Consumer < Item , Fut > , { type Output = C :: Output ; async fn send (self : Pin < & mut Self > , future : Fut) -> ConsumerState { let this = self . project () ; * this . count += 1 ; let state = this . inner . send (future) . await ; if this . count >= this . limit { ConsumerState :: Break } else { state } } async fn progress (self : Pin < & mut Self >) -> ConsumerState { let this = self . project () ; this . inner . progress () . await } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let this = self . project () ; this . inner . flush () . await } }
};
}
