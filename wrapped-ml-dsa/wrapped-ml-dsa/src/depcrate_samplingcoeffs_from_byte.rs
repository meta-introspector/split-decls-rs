// Generated macro for coeffs_from_byte (function)
macro_rules! Depcrate_samplingcoeffs_from_byte {
() => {
// Module: crate::sampling
// Provides: {"coeffs_from_byte"}
// Dependencies: {}
fn coeffs_from_byte (z : u8 , eta : Eta) -> (Option < Elem > , Option < Elem >) { (coeff_from_half_byte (z & 0x0F , eta) , coeff_from_half_byte (z >> 4 , eta) ,) }
};
}
