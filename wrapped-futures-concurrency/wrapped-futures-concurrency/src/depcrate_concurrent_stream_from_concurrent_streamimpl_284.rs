// Generated macro for impl_284 (impl)
macro_rules! Depcrate_concurrent_stream_from_concurrent_streamimpl_284 {
() => {
// Module: crate::concurrent_stream::from_concurrent_stream
// Provides: {"impl_284"}
// Dependencies: {}
impl < Item , Fut > Consumer < Item , Fut > for VecConsumer < '_ , Fut > where Fut : Future < Output = Item > , { type Output = () ; async fn send (self : Pin < & mut Self > , future : Fut) -> super :: ConsumerState { let mut this = self . project () ; this . group . as_mut () . push (future) ; ConsumerState :: Continue } async fn progress (self : Pin < & mut Self >) -> super :: ConsumerState { let mut this = self . project () ; while let Some (item) = this . group . next () . await { this . output . push (item) ; } ConsumerState :: Empty } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let mut this = self . project () ; while let Some (item) = this . group . next () . await { this . output . push (item) ; } } }
};
}
