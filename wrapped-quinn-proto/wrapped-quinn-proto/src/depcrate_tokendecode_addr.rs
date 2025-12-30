// Generated macro for decode_addr (function)
macro_rules! Depcrate_tokendecode_addr {
() => {
// Module: crate::token
// Provides: {"decode_addr"}
// Dependencies: {}
fn decode_addr < B : Buf > (buf : & mut B) -> Option < SocketAddr > { let ip = decode_ip (buf) ? ; let port = buf . get () . ok () ? ; Some (SocketAddr :: new (ip , port)) }
};
}
