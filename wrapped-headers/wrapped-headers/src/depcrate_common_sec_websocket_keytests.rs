// Generated macro for tests (module)
macro_rules! Depcrate_common_sec_websocket_keytests {
() => {
// Module: crate::common::sec_websocket_key
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn from_bytes () { let bytes : [u8 ; 16] = [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16] ; let _ = SecWebsocketKey :: from (bytes) ; } }
};
}
