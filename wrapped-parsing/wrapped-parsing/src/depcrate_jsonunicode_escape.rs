// Generated macro for unicode_escape (function)
macro_rules! Depcrate_jsonunicode_escape {
() => {
// Module: crate::json
// Provides: {"unicode_escape"}
// Dependencies: {}
fn unicode_escape (input : & str) -> IResult < & str , char > { map_opt (alt ((map (verify (u16_hex , | cp | ! (0xD800 .. 0xE000) . contains (cp)) , | cp | { cp as u32 }) , map (verify (separated_pair (u16_hex , tag ("\\u") , u16_hex) , | (high , low) | (0xD800 .. 0xDC00) . contains (high) && (0xDC00 .. 0xE000) . contains (low) ,) , | (high , low) | { let high_ten = (high as u32) - 0xD800 ; let low_ten = (low as u32) - 0xDC00 ; (high_ten << 10) + low_ten + 0x10000 } ,) ,)) , std :: char :: from_u32 ,) . parse (input) }
};
}
