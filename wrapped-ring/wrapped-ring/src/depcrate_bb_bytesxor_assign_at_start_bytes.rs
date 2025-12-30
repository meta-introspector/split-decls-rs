// Generated macro for xor_assign_at_start_bytes (function)
macro_rules! Depcrate_bb_bytesxor_assign_at_start_bytes {
() => {
// Module: crate::bb::bytes
// Provides: {"xor_assign_at_start_bytes"}
// Dependencies: {}
# [doc = " XORs the first N bytes of `b` into `a`, where N is `a.len().min(b.len())`."] # [inline (always)] pub (crate) fn xor_assign_at_start_bytes < 'a > (a : impl IntoIterator < Item = & 'a mut u8 > , b : impl IntoIterator < Item = & 'a u8 > ,) { a . into_iter () . zip (b) . for_each (| (a , b) | * a ^= * b) ; }
};
}
