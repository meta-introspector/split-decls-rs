// Generated macro for impl_50 (impl)
macro_rules! Depcrate_encodeimpl_50 {
() => {
// Module: crate::encode
// Provides: {"impl_50"}
// Dependencies: {}
impl < K , A , B > BitPack < A , B > for Vector < K > where K : ArraySize , (A , B) : RangeEncodingSize , RangeEncodingBits < A , B > : VectorEncodingSize < K > , { type PackedSize = RangeEncodedVectorSize < A , B , K > ; fn pack (& self) -> RangeEncodedVector < A , B , K > { let polys = self . 0 . iter () . map (| x | BitPack :: < A , B > :: pack (x)) . collect () ; RangeEncodingBits :: < A , B > :: flatten (polys) } fn unpack (enc : & RangeEncodedVector < A , B , K >) -> Self { let unfold = RangeEncodingBits :: < A , B > :: unflatten (enc) ; Self (unfold . into_iter () . map (| x | < Polynomial as BitPack < A , B > > :: unpack (x)) . collect () ,) } }
};
}
