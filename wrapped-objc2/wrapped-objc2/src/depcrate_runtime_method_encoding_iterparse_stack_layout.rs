// Generated macro for parse_stack_layout (function)
macro_rules! Depcrate_runtime_method_encoding_iterparse_stack_layout {
() => {
// Module: crate::runtime::method_encoding_iter
// Provides: {"parse_stack_layout"}
// Dependencies: {}
fn parse_stack_layout (s : & mut & str) -> Result < Option < isize > , ParseIntError > { let rest = s . trim_start_matches (| c : char | c . is_ascii_digit () || c == '-' || c == '+') ; let stack_layout = & s [.. s . len () - rest . len ()] ; * s = rest ; if stack_layout . is_empty () { return Ok (None) ; } stack_layout . parse () . map (Some) }
};
}
