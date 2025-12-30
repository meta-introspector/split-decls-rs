// Generated macro for is_zero (function)
macro_rules! Depcrate_ctis_zero {
() => {
// Module: crate::ct
// Provides: {"is_zero"}
// Dependencies: {}
# [doc = " Constant-time test that a given byte slice contains only zeroes."] # [inline] pub (crate) fn is_zero (n : & [u8]) -> Choice { let mut ret = Choice :: from (1) ; for byte in n { ret . conditional_assign (& Choice :: from (0) , byte . ct_ne (& 0)) ; } ret }
};
}
