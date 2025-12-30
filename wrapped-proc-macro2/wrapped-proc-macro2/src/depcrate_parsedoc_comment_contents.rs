// Generated macro for doc_comment_contents (function)
macro_rules! Depcrate_parsedoc_comment_contents {
() => {
// Module: crate::parse
// Provides: {"doc_comment_contents"}
// Dependencies: {}
fn doc_comment_contents (input : Cursor) -> PResult < (& str , bool) > { if input . starts_with ("//!") { let input = input . advance (3) ; let (input , s) = take_until_newline_or_eof (input) ; Ok ((input , (s , true))) } else if input . starts_with ("/*!") { let (input , s) = block_comment (input) ? ; Ok ((input , (& s [3 .. s . len () - 2] , true))) } else if input . starts_with ("///") { let input = input . advance (3) ; if input . starts_with_char ('/') { return Err (Reject) ; } let (input , s) = take_until_newline_or_eof (input) ; Ok ((input , (s , false))) } else if input . starts_with ("/**") && ! input . rest [3 ..] . starts_with ('*') { let (input , s) = block_comment (input) ? ; Ok ((input , (& s [3 .. s . len () - 2] , false))) } else { Err (Reject) } }
};
}
