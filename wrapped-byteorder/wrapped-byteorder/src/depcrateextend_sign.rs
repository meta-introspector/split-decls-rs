// Generated macro for extend_sign (function)
macro_rules! Depcrateextend_sign {
() => {
// Module: crate
// Provides: {"extend_sign"}
// Dependencies: {}
# [inline] fn extend_sign (val : u64 , nbytes : usize) -> i64 { let shift = (8 - nbytes) * 8 ; (val << shift) as i64 >> shift }
};
}
