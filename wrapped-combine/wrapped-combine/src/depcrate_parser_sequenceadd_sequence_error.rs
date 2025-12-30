// Generated macro for add_sequence_error (function)
macro_rules! Depcrate_parser_sequenceadd_sequence_error {
() => {
// Module: crate::parser::sequence
// Provides: {"add_sequence_error"}
// Dependencies: {}
fn add_sequence_error < Input > (i : & mut usize , first_empty_parser : usize , inner_offset : ErrorOffset , err : & mut Tracked < Input :: Error > , parser : & mut impl Parser < Input > ,) -> bool where Input : Stream , { if * i + 1 == first_empty_parser { Parser :: add_committed_expected_error (parser , err) ; } if * i >= first_empty_parser { if err . offset <= ErrorOffset (1) { err . offset = inner_offset ; } Parser :: add_error (parser , err) ; if err . offset <= ErrorOffset (1) { return false ; } } err . offset = ErrorOffset (err . offset . 0 . saturating_sub (Parser :: parser_count (parser) . 0)) ; * i += 1 ; true }
};
}
