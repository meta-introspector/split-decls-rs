// Generated macro for coeff_from_three_bytes (function)
macro_rules! Depcrate_samplingcoeff_from_three_bytes {
() => {
// Module: crate::sampling
// Provides: {"coeff_from_three_bytes"}
// Dependencies: {}
fn coeff_from_three_bytes (b : [u8 ; 3]) -> Option < Elem > { let b0 : Int = b [0] . into () ; let b1 : Int = b [1] . into () ; let b2 : Int = b [2] . into () ; let b2p = if b2 > 127 { b2 - 128 } else { b2 } ; let z = (b2p << 16) + (b1 << 8) + b0 ; (z < BaseField :: Q) . then_some (Elem :: new (z)) }
};
}
