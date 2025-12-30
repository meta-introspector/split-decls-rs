// Generated macro for host (function)
macro_rules! Depcrate_uri_authorityhost {
() => {
// Module: crate::uri::authority
// Provides: {"host"}
// Dependencies: {}
fn host (auth : & str) -> & str { let host_port = auth . rsplit ('@') . next () . expect ("split always has at least 1 item") ; if host_port . as_bytes () [0] == b'[' { let i = host_port . find (']') . expect ("parsing should validate brackets") ; & host_port [0 .. i + 1] } else { host_port . split (':') . next () . expect ("split always has at least 1 item") } }
};
}
