// Generated macro for count_digits (function)
macro_rules! Depcrate_float_literalcount_digits {
() => {
// Module: crate::float_literal
// Provides: {"count_digits"}
// Dependencies: {}
# [doc = " Counts the digits excluding leading zeros"] # [must_use] fn count_digits (s : & str) -> usize { s . chars () . filter (| c | * c != '-' && * c != '.') . take_while (| c | * c != 'e' && * c != 'E') . fold (0 , | count , c | { if c == '0' && count == 0 { count } else { count + 1 } }) }
};
}
