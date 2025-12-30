// Generated macro for glyphless_mask (function)
macro_rules! Depcrate_uts46glyphless_mask {
() => {
// Module: crate::uts46
// Provides: {"glyphless_mask"}
// Dependencies: {}
# [doc = " Computes the mask for glyphless ASCII."] const fn glyphless_mask () -> u128 { let mut accu = 0u128 ; let mut b = 0u8 ; while b < 128 { if (b <= b' ') || (b == 0x7F) { accu |= 1u128 << b ; } b += 1 ; } accu }
};
}
