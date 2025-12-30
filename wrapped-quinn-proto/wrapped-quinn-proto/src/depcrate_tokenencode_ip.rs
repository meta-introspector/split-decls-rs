// Generated macro for encode_ip (function)
macro_rules! Depcrate_tokenencode_ip {
() => {
// Module: crate::token
// Provides: {"encode_ip"}
// Dependencies: {}
fn encode_ip (buf : & mut Vec < u8 > , ip : IpAddr) { match ip { IpAddr :: V4 (x) => { buf . put_u8 (0) ; buf . put_slice (& x . octets ()) ; } IpAddr :: V6 (x) => { buf . put_u8 (1) ; buf . put_slice (& x . octets ()) ; } } }
};
}
