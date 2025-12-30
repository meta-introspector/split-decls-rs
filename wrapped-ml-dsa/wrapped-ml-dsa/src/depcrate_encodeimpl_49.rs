// Generated macro for impl_49 (impl)
macro_rules! Depcrate_encodeimpl_49 {
() => {
// Module: crate::encode
// Provides: {"impl_49"}
// Dependencies: {}
impl < A , B > BitPack < A , B > for Polynomial where (A , B) : RangeEncodingSize , { type PackedSize = RangeEncodedPolynomialSize < A , B > ; fn pack (& self) -> RangeEncodedPolynomial < A , B > { let a = Elem :: new (RangeMin :: < A , B > :: U32) ; let b = Elem :: new (RangeMax :: < A , B > :: U32) ; let to_encode = Self :: new (self . 0 . iter () . map (| w | { assert ! (w . 0 <= b . 0 || w . 0 >= (- a) . 0) ; b - * w }) . collect () ,) ; Encode :: < RangeEncodingBits < A , B > > :: encode (& to_encode) } fn unpack (enc : & RangeEncodedPolynomial < A , B >) -> Self { let a = Elem :: new (RangeMin :: < A , B > :: U32) ; let b = Elem :: new (RangeMax :: < A , B > :: U32) ; let mut decoded : Self = Encode :: < RangeEncodingBits < A , B > > :: decode (enc) ; for z in & mut decoded . 0 { assert ! (z . 0 <= (a + b) . 0) ; * z = b - * z ; } decoded } }
};
}
