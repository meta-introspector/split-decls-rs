// Generated macro for u16_hex (function)
macro_rules! Depcrate_jsonu16_hex {
() => {
// Module: crate::json
// Provides: {"u16_hex"}
// Dependencies: {}
fn u16_hex (input : & str) -> IResult < & str , u16 > { map_res (take (4usize) , | s | u16 :: from_str_radix (s , 16)) . parse (input) }
};
}
