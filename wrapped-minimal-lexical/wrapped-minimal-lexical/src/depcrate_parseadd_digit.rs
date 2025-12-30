// Generated macro for add_digit (function)
macro_rules! Depcrate_parseadd_digit {
() => {
// Module: crate::parse
// Provides: {"add_digit"}
// Dependencies: {}
# [inline] pub fn add_digit (value : u64 , digit : u8) -> Option < u64 > { value . checked_mul (10) ? . checked_add (digit as u64) }
};
}
