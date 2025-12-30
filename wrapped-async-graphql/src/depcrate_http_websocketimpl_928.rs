// Generated macro for impl_928 (impl)
macro_rules! Depcrate_http_websocketimpl_928 {
() => {
// Module: crate::http::websocket
// Provides: {"impl_928"}
// Dependencies: {}
impl WsMessage { # [doc = " Returns the contained [WsMessage::Text] value, consuming the `self`"] # [doc = " value."] # [doc = ""] # [doc = " Because this function may panic, its use is generally discouraged."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the self value not equals [WsMessage::Text]."] pub fn unwrap_text (self) -> String { match self { Self :: Text (text) => text , Self :: Close (_ , _) => panic ! ("Not a text message") , } } # [doc = " Returns the contained [WsMessage::Close] value, consuming the `self`"] # [doc = " value."] # [doc = ""] # [doc = " Because this function may panic, its use is generally discouraged."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the self value not equals [WsMessage::Close]."] pub fn unwrap_close (self) -> (u16 , String) { match self { Self :: Close (code , msg) => (code , msg) , Self :: Text (_) => panic ! ("Not a close message") , } } }
};
}
