// Generated macro for impl_71 (impl)
macro_rules! Depcrate_graphql_wsimpl_71 {
() => {
// Module: crate::graphql_ws
// Provides: {"impl_71"}
// Dependencies: {}
impl < S , I > Stream for Connection < S , I > where S : Schema , I : Init < S :: ScalarValue , S :: Context > , { type Item = ServerMessage < S :: ScalarValue > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Option < Self :: Item > > { self . stream_waker = Some (cx . waker () . clone ()) ; if let ConnectionSinkState :: Closed = self . sink_state { return Poll :: Ready (None) ; } if ! self . reactions . is_empty () { match Pin :: new (& mut self . reactions) . poll_next (cx) { Poll :: Ready (Some (reaction)) => match reaction { Reaction :: ServerMessage (msg) => return Poll :: Ready (Some (msg)) , Reaction :: EndStream => return Poll :: Ready (None) , } , Poll :: Ready (None) => { self . reactions = SelectAll :: new () ; } _ => () , } } Poll :: Pending } }
};
}
