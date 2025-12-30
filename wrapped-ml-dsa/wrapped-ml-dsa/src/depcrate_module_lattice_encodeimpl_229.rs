// Generated macro for impl_229 (impl)
macro_rules! Depcrate_module_lattice_encodeimpl_229 {
() => {
// Module: crate::module_lattice::encode
// Provides: {"impl_229"}
// Dependencies: {}
impl < D , K > VectorEncodingSize < K > for D where D : EncodingSize , K : ArraySize , D :: EncodedPolynomialSize : Mul < K > , Prod < D :: EncodedPolynomialSize , K > : ArraySize + Div < K , Output = D :: EncodedPolynomialSize > + Rem < K , Output = U0 > , { type EncodedVectorSize = Prod < D :: EncodedPolynomialSize , K > ; fn flatten (polys : Array < EncodedPolynomial < Self > , K >) -> EncodedVector < Self , K > { polys . flatten () } fn unflatten (vec : & EncodedVector < Self , K >) -> Array < & EncodedPolynomial < Self > , K > { vec . unflatten () } }
};
}
