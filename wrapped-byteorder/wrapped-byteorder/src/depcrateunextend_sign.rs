// Generated macro for unextend_sign (function)
macro_rules! Depcrateunextend_sign {
() => {
// Module: crate
// Provides: {"unextend_sign"}
// Dependencies: {}
# [inline] fn unextend_sign (val : i64 , nbytes : usize) -> u64 { let shift = (8 - nbytes) * 8 ; (val << shift) as u64 >> shift }
};
}
