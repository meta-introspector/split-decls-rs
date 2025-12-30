// Generated macro for impl_36 (impl)
macro_rules! Depcrate_graphql_transport_wsimpl_36 {
() => {
// Module: crate::graphql_transport_ws
// Provides: {"impl_36"}
// Dependencies: {}
impl < S , I > Stream for Connection < S , I > where S : Schema , I : Init < S :: ScalarValue , S :: Context > , { type Item = Output < S :: ScalarValue > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Option < Self :: Item > > { self . stream_waker = Some (cx . waker () . clone ()) ; if self . stream_terminated { return Poll :: Ready (None) ; } if ! self . reactions . is_empty () { match Pin :: new (& mut self . reactions) . poll_next (cx) { Poll :: Ready (Some (Output :: Close { code , message })) => { self . stream_terminated = true ; return Poll :: Ready (Some (Output :: Close { code , message })) ; } Poll :: Ready (Some (reaction)) => return Poll :: Ready (Some (reaction)) , Poll :: Ready (None) => { self . reactions = SelectAll :: new () ; } _ => () , } } if let ConnectionSinkState :: Closed = self . sink_state { return Poll :: Ready (None) ; } Poll :: Pending } }
};
}
