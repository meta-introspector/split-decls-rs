// Generated macro for assemble_q (function)
macro_rules! Depcrate_linalg_householderassemble_q {
() => {
// Module: crate::linalg::householder
// Provides: {"assemble_q"}
// Dependencies: {}
# [doc = " Computes the orthogonal transformation described by the elementary reflector axii stored on"] # [doc = " the lower-diagonal element of the given matrix."] # [doc = " matrices."] # [doc (hidden)] pub fn assemble_q < T : ComplexField , D : Dim > (m : & OMatrix < T , D , D > , signs : & [T]) -> OMatrix < T , D , D > where DefaultAllocator : Allocator < D , D > , { assert ! (m . is_square ()) ; let dim = m . shape_generic () . 0 ; let mut res = OMatrix :: identity_generic (dim , dim) ; for i in (0 .. dim . value () - 1) . rev () { let axis = m . view_range (i + 1 .. , i) ; let refl = Reflection :: new (Unit :: new_unchecked (axis) , T :: zero ()) ; let mut res_rows = res . view_range_mut (i + 1 .. , i ..) ; refl . reflect_with_sign (& mut res_rows , signs [i] . clone () . signum ()) ; } res }
};
}
