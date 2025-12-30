// Generated macro for LinearCombination (trait)
macro_rules! Depcrate_opsLinearCombination {
() => {
// Module: crate::ops
// Provides: {"LinearCombination"}
// Dependencies: {}
# [doc = " Linear combination."] # [doc = ""] # [doc = " This trait enables optimized implementations of linear combinations (e.g. Shamir's Trick)."] # [doc = ""] # [doc = " It's generic around `PointsAndScalars` to allow overlapping impls. For example, const generic"] # [doc = " impls can use the input size to determine the size needed to store temporary variables."] pub trait LinearCombination < PointsAndScalars > : CurveGroup where PointsAndScalars : AsRef < [(Self , Self :: Scalar)] > + ? Sized , { # [doc = " Calculates `x1 * k1 + ... + xn * kn`."] fn lincomb (points_and_scalars : & PointsAndScalars) -> Self { points_and_scalars . as_ref () . iter () . copied () . map (| (point , scalar) | point * scalar) . sum () } }
};
}
