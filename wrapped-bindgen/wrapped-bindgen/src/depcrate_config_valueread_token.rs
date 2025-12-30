// Generated macro for read_token (function)
macro_rules! Depcrate_config_valueread_token {
() => {
// Module: crate::config::value
// Provides: {"read_token"}
// Dependencies: {}
fn read_token (input : & str , token : u8) -> & str { for (pos , c) in input . bytes () . enumerate () { if c == token { return & input [pos + 1 ..] ; } else if c != b' ' && c != b',' { break ; } } panic ! ("`{}` expected" , token . escape_ascii ()) ; }
};
}
