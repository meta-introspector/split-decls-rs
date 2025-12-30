// Generated macro for reflection_axis_mut (function)
macro_rules! Depcrate_linalg_householderreflection_axis_mut {
() => {
// Module: crate::linalg::householder
// Provides: {"reflection_axis_mut"}
// Dependencies: {}
# [doc = " Replaces `column` by the axis of the householder reflection that transforms `column` into"] # [doc = " `(+/-|column|, 0, ..., 0)`."] # [doc = ""] # [doc = " The unit-length axis is output to `column`. Returns what would be the first component of"] # [doc = " `column` after reflection and `false` if no reflection was necessary."] # [doc (hidden)] # [inline (always)] pub fn reflection_axis_mut < T : ComplexField , D : Dim , S : StorageMut < T , D > > (column : & mut Vector < T , D , S > ,) -> (T , bool) { let reflection_sq_norm = column . norm_squared () ; let reflection_norm = reflection_sq_norm . clone () . sqrt () ; let factor ; let signed_norm ; unsafe { let (modulus , sign) = column . vget_unchecked (0) . clone () . to_exp () ; signed_norm = sign . scale (reflection_norm . clone ()) ; factor = (reflection_sq_norm + modulus * reflection_norm) * crate :: convert (2.0) ; * column . vget_unchecked_mut (0) += signed_norm . clone () ; } ; if ! factor . is_zero () { column . unscale_mut (factor . sqrt ()) ; let _ = column . normalize_mut () ; (- signed_norm , true) } else { (signed_norm , false) } }
};
}
