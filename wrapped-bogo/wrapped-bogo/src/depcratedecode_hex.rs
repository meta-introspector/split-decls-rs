// Generated macro for decode_hex (function)
macro_rules! Depcratedecode_hex {
() => {
// Module: crate
// Provides: {"decode_hex"}
// Dependencies: {}
fn decode_hex (hex : & str) -> Vec < u8 > { (0 .. hex . len ()) . step_by (2) . map (| i | u8 :: from_str_radix (& hex [i .. i + 2] , 16) . unwrap ()) . inspect (| x | println ! ("item {x:?}")) . collect () }
};
}
