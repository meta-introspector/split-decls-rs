// Generated macro for impl_133 (impl)
macro_rules! Depcrate_websocket_futuresimpl_133 {
() => {
// Module: crate::websocket::futures
// Provides: {"impl_133"}
// Dependencies: {}
impl Stream for WebSocket { type Item = Result < Message , WebSocketError > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let msg = ready ! (self . project () . message_receiver . poll_next (cx)) ; match msg { Some (StreamMessage :: Message (msg)) => Poll :: Ready (Some (Ok (msg))) , Some (StreamMessage :: ErrorEvent) => { Poll :: Ready (Some (Err (WebSocketError :: ConnectionError))) } Some (StreamMessage :: CloseEvent (e)) => { Poll :: Ready (Some (Err (WebSocketError :: ConnectionClose (e)))) } Some (StreamMessage :: ConnectionClose) => Poll :: Ready (None) , None => Poll :: Ready (None) , } } }
};
}
