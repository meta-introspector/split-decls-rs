// Generated macro for extend_sign128 (function)
macro_rules! Depcrateextend_sign128 {
() => {
// Module: crate
// Provides: {"extend_sign128"}
// Dependencies: {}
# [inline] fn extend_sign128 (val : u128 , nbytes : usize) -> i128 { let shift = (16 - nbytes) * 8 ; (val << shift) as i128 >> shift }
};
}
