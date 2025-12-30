// Generated macro for number (function)
macro_rules! Depcrate_multi_testsnumber {
() => {
// Module: crate::multi::tests
// Provides: {"number"}
// Dependencies: {}
fn number (i : & [u8]) -> IResult < & [u8] , u32 > { use crate :: combinator :: map_res ; map_res (map_res (digit , str :: from_utf8) , FromStr :: from_str) . parse (i) }
};
}
