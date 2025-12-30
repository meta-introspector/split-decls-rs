// Generated macro for clear_column_unchecked (function)
macro_rules! Depcrate_linalg_householderclear_column_unchecked {
() => {
// Module: crate::linalg::householder
// Provides: {"clear_column_unchecked"}
// Dependencies: {}
# [doc = " Uses an householder reflection to zero out the `icol`-th column, starting with the `shift + 1`-th"] # [doc = " subdiagonal element."] # [doc = ""] # [doc = " Returns the signed norm of the column."] # [doc (hidden)] # [must_use] pub fn clear_column_unchecked < T : ComplexField , R : Dim , C : Dim > (matrix : & mut OMatrix < T , R , C > , icol : usize , shift : usize , bilateral : Option < & mut OVector < T , R > > ,) -> T where DefaultAllocator : Allocator < R , C > + Allocator < R > , { let (mut left , mut right) = matrix . columns_range_pair_mut (icol , icol + 1 ..) ; let mut axis = left . rows_range_mut (icol + shift ..) ; let (reflection_norm , not_zero) = reflection_axis_mut (& mut axis) ; if not_zero { let refl = Reflection :: new (Unit :: new_unchecked (axis) , T :: zero ()) ; let sign = reflection_norm . clone () . signum () ; if let Some (work) = bilateral { refl . reflect_rows_with_sign (& mut right , work , sign . clone ()) ; } refl . reflect_with_sign (& mut right . rows_range_mut (icol + shift ..) , sign . conjugate ()) ; } reflection_norm }
};
}
