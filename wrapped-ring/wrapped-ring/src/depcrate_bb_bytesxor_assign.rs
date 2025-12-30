// Generated macro for xor_assign (function)
macro_rules! Depcrate_bb_bytesxor_assign {
() => {
// Module: crate::bb::bytes
// Provides: {"xor_assign"}
// Dependencies: {}
# [inline (always)] pub (crate) fn xor_assign < 'a > (a : impl IntoIterator < Item = & 'a mut u8 > , b : u8) { a . into_iter () . for_each (| a | * a ^= b) ; }
};
}
