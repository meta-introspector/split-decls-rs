// Generated macro for parse_u64 (function)
macro_rules! Depcrate_frame_headersparse_u64 {
() => {
// Module: crate::frame::headers
// Provides: {"parse_u64"}
// Dependencies: {}
pub fn parse_u64 (src : & [u8]) -> Result < u64 , ParseU64Error > { if src . len () > 19 { return Err (ParseU64Error) ; } let mut ret = 0 ; for & d in src { if d < b'0' || d > b'9' { return Err (ParseU64Error) ; } ret *= 10 ; ret += (d - b'0') as u64 ; } Ok (ret) }
};
}
