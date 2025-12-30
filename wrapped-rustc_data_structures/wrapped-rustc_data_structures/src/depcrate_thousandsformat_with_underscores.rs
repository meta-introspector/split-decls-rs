// Generated macro for format_with_underscores (function)
macro_rules! Depcrate_thousandsformat_with_underscores {
() => {
// Module: crate::thousands
// Provides: {"format_with_underscores"}
// Dependencies: {}
fn format_with_underscores (mut s : String) -> String { let start = if s . starts_with ('-') { 1 } else { 0 } ; let non_digit = s [start ..] . find (| c : char | ! c . is_digit (10)) ; let end = if let Some (non_digit) = non_digit { start + non_digit } else { s . len () } ; let mut i = end ; while i > start + 3 { i -= 3 ; s . insert (i , '_') ; } s }
};
}
