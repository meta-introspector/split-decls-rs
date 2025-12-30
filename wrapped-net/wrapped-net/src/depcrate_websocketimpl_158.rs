// Generated macro for impl_158 (impl)
macro_rules! Depcrate_websocketimpl_158 {
() => {
// Module: crate::websocket
// Provides: {"impl_158"}
// Dependencies: {}
impl fmt :: Display for WebSocketError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { WebSocketError :: ConnectionError => write ! (f , "WebSocket connection failed") , WebSocketError :: ConnectionClose (e) => write ! (f , "WebSocket Closed: code: {}, reason: {}" , e . code , e . reason) , WebSocketError :: MessageSendError (e) => write ! (f , "{e}") , } } }
};
}
