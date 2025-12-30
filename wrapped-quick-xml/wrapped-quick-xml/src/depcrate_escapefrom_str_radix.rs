// Generated macro for from_str_radix (function)
macro_rules! Depcrate_escapefrom_str_radix {
() => {
// Module: crate::escape
// Provides: {"from_str_radix"}
// Dependencies: {}
# [inline] fn from_str_radix (src : & str , radix : u32) -> Result < u32 , ParseCharRefError > { match src . as_bytes () . first () . copied () { Some (b'+') | Some (b'-') => Err (ParseCharRefError :: UnexpectedSign) , _ => u32 :: from_str_radix (src , radix) . map_err (ParseCharRefError :: InvalidNumber) , } }
};
}
