// Generated macro for read_req (function)
macro_rules! Depcrate_serverread_req {
() => {
// Module: crate::server
// Provides: {"read_req"}
// Dependencies: {}
async fn read_req (mut stream : quinn :: RecvStream) -> Result < u64 > { let mut buf = [0 ; 8] ; stream . read_exact (& mut buf) . await . context ("reading request") ? ; let n = u64 :: from_be_bytes (buf) ; debug ! ("got req for {} bytes on {}" , n , stream . id ()) ; drain_stream (stream) . await ? ; Ok (n) }
};
}
