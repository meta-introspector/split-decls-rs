// Generated macro for xor_assign_at_start (function)
macro_rules! Depcrate_bbxor_assign_at_start {
() => {
// Module: crate::bb
// Provides: {"xor_assign_at_start"}
// Dependencies: {}
# [doc = " XORs the first N words of `b` into `a`, where N is"] # [doc = " `a.len().min(b.len())`."] # [inline (always)] pub (crate) fn xor_assign_at_start < 'a > (a : impl IntoIterator < Item = & 'a mut Word > , b : impl IntoIterator < Item = & 'a Word > ,) { a . into_iter () . zip (b) . for_each (| (a , b) | * a ^= * b) ; }
};
}
