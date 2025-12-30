// Generated macro for EncodingSize (trait)
macro_rules! Depcrate_module_lattice_encodeEncodingSize {
() => {
// Module: crate::module_lattice::encode
// Provides: {"EncodingSize"}
// Dependencies: {}
# [doc = " An integer that can describe encoded polynomials."] pub trait EncodingSize : ArraySize { type EncodedPolynomialSize : ArraySize ; type ValueStep : ArraySize ; type ByteStep : ArraySize ; }
};
}
