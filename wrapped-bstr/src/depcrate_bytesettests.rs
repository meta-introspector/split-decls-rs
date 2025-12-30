// Generated macro for tests (module)
macro_rules! Depcrate_bytesettests {
() => {
// Module: crate::byteset
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "std" , not (miri)))] mod tests { use alloc :: vec :: Vec ; quickcheck :: quickcheck ! { fn qc_byteset_forward_matches_naive (haystack : Vec < u8 >, needles : Vec < u8 >) -> bool { super :: find (& haystack , & needles) == haystack . iter () . position (| b | needles . contains (b)) } fn qc_byteset_backwards_matches_naive (haystack : Vec < u8 >, needles : Vec < u8 >) -> bool { super :: rfind (& haystack , & needles) == haystack . iter () . rposition (| b | needles . contains (b)) } fn qc_byteset_forward_not_matches_naive (haystack : Vec < u8 >, needles : Vec < u8 >) -> bool { super :: find_not (& haystack , & needles) == haystack . iter () . position (| b | ! needles . contains (b)) } fn qc_byteset_backwards_not_matches_naive (haystack : Vec < u8 >, needles : Vec < u8 >) -> bool { super :: rfind_not (& haystack , & needles) == haystack . iter () . rposition (| b | ! needles . contains (b)) } } }
};
}
