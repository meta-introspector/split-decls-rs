// Generated macro for make_hint (function)
macro_rules! Depcrate_hintmake_hint {
() => {
// Module: crate::hint
// Provides: {"make_hint"}
// Dependencies: {}
fn make_hint < TwoGamma2 : Unsigned > (z : Elem , r : Elem) -> bool { let r1 = r . high_bits :: < TwoGamma2 > () ; let v1 = (r + z) . high_bits :: < TwoGamma2 > () ; r1 != v1 }
};
}
