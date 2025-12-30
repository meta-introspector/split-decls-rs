// Generated macro for impl_234 (impl)
macro_rules! Depcrate_module_lattice_encodeimpl_234 {
() => {
// Module: crate::module_lattice::encode
// Provides: {"impl_234"}
// Dependencies: {}
impl < F , D , K > Encode < D > for Vector < F , K > where F : Field , K : ArraySize , D : VectorEncodingSize < K > , { type EncodedSize = D :: EncodedVectorSize ; fn encode (& self) -> Array < u8 , Self :: EncodedSize > { let polys = self . 0 . iter () . map (| x | Encode :: < D > :: encode (x)) . collect () ; < D as VectorEncodingSize < K > > :: flatten (polys) } fn decode (enc : & Array < u8 , Self :: EncodedSize >) -> Self { let unfold = < D as VectorEncodingSize < K > > :: unflatten (enc) ; Self (unfold . iter () . map (| & x | < Polynomial < F > as Encode < D > > :: decode (x)) . collect () ,) } }
};
}
