// Generated macro for reverse_bits_fallback (function)
macro_rules! Depcrate_intreverse_bits_fallback {
() => {
// Module: crate::int
// Provides: {"reverse_bits_fallback"}
// Dependencies: {}
fn reverse_bits_fallback < P : PrimInt > (i : P) -> P { let rep_01 : P = one_per_byte () ; let rep_03 = (rep_01 << 1) | rep_01 ; let rep_05 = (rep_01 << 2) | rep_01 ; let rep_0f = (rep_03 << 2) | rep_03 ; let rep_33 = (rep_03 << 4) | rep_03 ; let rep_55 = (rep_05 << 4) | rep_05 ; let mut ret = i . swap_bytes () ; ret = ((ret & rep_0f) << 4) | ((ret >> 4) & rep_0f) ; ret = ((ret & rep_33) << 2) | ((ret >> 2) & rep_33) ; ret = ((ret & rep_55) << 1) | ((ret >> 1) & rep_55) ; ret }
};
}
