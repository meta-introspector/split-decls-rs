// Generated macro for encode_addr (function)
macro_rules! Depcrate_tokenencode_addr {
() => {
// Module: crate::token
// Provides: {"encode_addr"}
// Dependencies: {}
fn encode_addr (buf : & mut Vec < u8 > , address : SocketAddr) { encode_ip (buf , address . ip ()) ; buf . put_u16 (address . port ()) ; }
};
}
