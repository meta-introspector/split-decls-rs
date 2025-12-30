// Generated macro for unextend_sign128 (function)
macro_rules! Depcrateunextend_sign128 {
() => {
// Module: crate
// Provides: {"unextend_sign128"}
// Dependencies: {}
# [inline] fn unextend_sign128 (val : i128 , nbytes : usize) -> u128 { let shift = (16 - nbytes) * 8 ; (val << shift) as u128 >> shift }
};
}
