// Generated macro for impl_93 (impl)
macro_rules! Depcrate_projectiveimpl_93 {
() => {
// Module: crate::projective
// Provides: {"impl_93"}
// Dependencies: {}
impl < C , const N : usize > LinearCombination < [(Self , Scalar < C >) ; N] > for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { fn lincomb (points_and_scalars : & [(Self , Scalar < C >) ; N]) -> Self { let mut ks : [_ ; N] = array :: from_fn (| index | { Into :: < C :: Uint > :: into (points_and_scalars [index] . 1) . to_le_byte_array () }) ; let mut pcs : [_ ; N] = array :: from_fn (| index | LookupTable :: new (points_and_scalars [index] . 0)) ; lincomb :: < C > (& mut ks , & mut pcs) } }
};
}
