// Generated macro for parse_message (function)
macro_rules! Depcrate_websocket_futuresparse_message {
() => {
// Module: crate::websocket::futures
// Provides: {"parse_message"}
// Dependencies: {}
fn parse_message (event : MessageEvent) -> Message { if let Ok (array_buffer) = event . data () . dyn_into :: < js_sys :: ArrayBuffer > () { let array = js_sys :: Uint8Array :: new (& array_buffer) ; Message :: Bytes (array . to_vec ()) } else if let Ok (txt) = event . data () . dyn_into :: < js_sys :: JsString > () { Message :: Text (String :: from (& txt)) } else { unreachable ! ("message event, received Unknown: {:?}" , event . data ()) ; } }
};
}
