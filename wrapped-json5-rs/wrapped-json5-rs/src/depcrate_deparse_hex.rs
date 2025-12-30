// Generated macro for parse_hex (function)
macro_rules! Depcrate_deparse_hex {
() => {
// Module: crate::de
// Provides: {"parse_hex"}
// Dependencies: {}
fn parse_hex (s : & str) -> Result < u32 > { u32 :: from_str_radix (s , 16) . or_else (| _ | Err (de :: Error :: custom ("error parsing hex"))) }
};
}
