// Generated macro for impl_40 (impl)
macro_rules! Depcrate_encodeimpl_40 {
() => {
// Module: crate::encode
// Provides: {"impl_40"}
// Dependencies: {}
impl < A , B > RangeEncodingSize for (A , B) where A : Unsigned + Add < B > , B : Unsigned , Sum < A , B > : Len , Length < Sum < A , B > > : EncodingSize , { type Min = A ; type Max = B ; type EncodingSize = Length < Sum < A , B > > ; }
};
}
