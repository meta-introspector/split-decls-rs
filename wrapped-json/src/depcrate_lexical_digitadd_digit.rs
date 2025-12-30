// Generated macro for add_digit (function)
macro_rules! Depcrate_lexical_digitadd_digit {
() => {
// Module: crate::lexical::digit
// Provides: {"add_digit"}
// Dependencies: {}
# [inline] pub (crate) fn add_digit (value : u64 , digit : u32) -> Option < u64 > { match value . checked_mul (10) { None => None , Some (n) => n . checked_add (digit as u64) , } }
};
}
