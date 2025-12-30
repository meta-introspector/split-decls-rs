// Generated macro for punct_char (function)
macro_rules! Depcrate_parsepunct_char {
() => {
// Module: crate::parse
// Provides: {"punct_char"}
// Dependencies: {}
fn punct_char (input : Cursor) -> PResult < char > { if input . starts_with ("//") || input . starts_with ("/*") { return Err (Reject) ; } let mut chars = input . chars () ; let Some (first) = chars . next () else { return Err (Reject) ; } ; let recognized = "~!@#$%^&*-=+|;:,<.>/?'" ; if recognized . contains (first) { Ok ((input . advance (first . len_utf8 ()) , first)) } else { Err (Reject) } }
};
}
