// Generated macro for impl_107 (impl)
macro_rules! Depcrate_devimpl_107 {
() => {
// Module: crate::dev
// Provides: {"impl_107"}
// Dependencies: {}
impl ToEncodedPoint < MockCurve > for AffinePoint { fn to_encoded_point (& self , compress : bool) -> EncodedPoint { match self { Self :: FixedBaseOutput (scalar) => EncodedPoint :: from_affine_coordinates (& scalar . to_repr () , & PSEUDO_COORDINATE_FIXED_BASE_MUL . into () , false ,) , Self :: Other (point) => { if compress == point . is_compressed () { * point } else { unimplemented ! () ; } } _ => unimplemented ! () , } } }
};
}
