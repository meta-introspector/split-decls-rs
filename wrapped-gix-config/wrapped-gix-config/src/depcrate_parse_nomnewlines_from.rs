// Generated macro for newlines_from (function)
macro_rules! Depcrate_parse_nomnewlines_from {
() => {
// Module: crate::parse::nom
// Provides: {"newlines_from"}
// Dependencies: {}
fn newlines_from (input : & [u8] , start : winnow :: stream :: Checkpoint < & [u8] , & [u8] >) -> usize { let offset = input . offset_from (& start) ; let mut start_input = input ; start_input . reset (& start) ; start_input . next_slice (offset) . iter () . filter (| c | * * c == b'\n') . count () }
};
}
