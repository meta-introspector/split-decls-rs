// Generated macro for parse_line (function)
macro_rules! Depcrate_parseparse_line {
() => {
// Module: crate::parse
// Provides: {"parse_line"}
// Dependencies: {}
fn parse_line (line : & str) -> Option < (u32 , u32 , & str) > { let (mut codepoint , rest) = line . split_once (';') ? ; let (lo , hi) ; codepoint = codepoint . trim () ; if let Some ((a , b)) = codepoint . split_once ("..") { lo = parse_codepoint (a) ? ; hi = parse_codepoint (b) ? ; } else { lo = parse_codepoint (codepoint) ? ; hi = lo ; } let name = rest . trim () . split ('#') . next () ? . trim_end () ; Some ((lo , hi , name)) }
};
}
