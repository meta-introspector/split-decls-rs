// Generated macro for VectorEncodingSize (trait)
macro_rules! Depcrate_module_lattice_encodeVectorEncodingSize {
() => {
// Module: crate::module_lattice::encode
// Provides: {"VectorEncodingSize"}
// Dependencies: {}
# [doc = " An integer that can describe encoded vectors."] pub trait VectorEncodingSize < K > : EncodingSize where K : ArraySize , { type EncodedVectorSize : ArraySize ; fn flatten (polys : Array < EncodedPolynomial < Self > , K >) -> EncodedVector < Self , K > ; fn unflatten (vec : & EncodedVector < Self , K >) -> Array < & EncodedPolynomial < Self > , K > ; }
};
}
