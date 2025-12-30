// Generated macro for decode_ip (function)
macro_rules! Depcrate_tokendecode_ip {
() => {
// Module: crate::token
// Provides: {"decode_ip"}
// Dependencies: {}
fn decode_ip < B : Buf > (buf : & mut B) -> Option < IpAddr > { match buf . get :: < u8 > () . ok () ? { 0 => buf . get () . ok () . map (IpAddr :: V4) , 1 => buf . get () . ok () . map (IpAddr :: V6) , _ => None , } }
};
}
