// Generated macro for ws_result_to_io_result (function)
macro_rules! Depcrate_websocket_io_utilws_result_to_io_result {
() => {
// Module: crate::websocket::io_util
// Provides: {"ws_result_to_io_result"}
// Dependencies: {}
fn ws_result_to_io_result (res : Result < () , WebSocketError >) -> io :: Result < () > { match res { Ok (()) => Ok (()) , Err (WebSocketError :: ConnectionClose (_)) => Ok (()) , Err (e) => Err (io :: Error :: new (io :: ErrorKind :: Other , e)) , } }
};
}
