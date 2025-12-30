// Generated macro for impl_236 (impl)
macro_rules! Depcrate_module_lattice_encodeimpl_236 {
() => {
// Module: crate::module_lattice::encode
// Provides: {"impl_236"}
// Dependencies: {}
impl < F , D , K > Encode < D > for NttVector < F , K > where F : Field , D : VectorEncodingSize < K > , K : ArraySize , { type EncodedSize = D :: EncodedVectorSize ; fn encode (& self) -> Array < u8 , Self :: EncodedSize > { let polys = self . 0 . iter () . map (| x | Encode :: < D > :: encode (x)) . collect () ; < D as VectorEncodingSize < K > > :: flatten (polys) } fn decode (enc : & Array < u8 , Self :: EncodedSize >) -> Self { let unfold = < D as VectorEncodingSize < K > > :: unflatten (enc) ; Self (unfold . iter () . map (| & x | < NttPolynomial < F > as Encode < D > > :: decode (x)) . collect () ,) } }
};
}
