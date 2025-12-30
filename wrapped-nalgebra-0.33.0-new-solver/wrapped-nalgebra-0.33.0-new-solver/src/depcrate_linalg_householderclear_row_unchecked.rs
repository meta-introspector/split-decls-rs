// Generated macro for clear_row_unchecked (function)
macro_rules! Depcrate_linalg_householderclear_row_unchecked {
() => {
// Module: crate::linalg::householder
// Provides: {"clear_row_unchecked"}
// Dependencies: {}
# [doc = " Uses an householder reflection to zero out the `irow`-th row, ending before the `shift + 1`-th"] # [doc = " superdiagonal element."] # [doc = ""] # [doc = " Returns the signed norm of the column."] # [doc (hidden)] # [must_use] pub fn clear_row_unchecked < T : ComplexField , R : Dim , C : Dim > (matrix : & mut OMatrix < T , R , C > , axis_packed : & mut OVector < T , C > , work : & mut OVector < T , R > , irow : usize , shift : usize ,) -> T where DefaultAllocator : Allocator < R , C > + Allocator < R > + Allocator < C > , { let (mut top , mut bottom) = matrix . rows_range_pair_mut (irow , irow + 1 ..) ; let mut axis = axis_packed . rows_range_mut (irow + shift ..) ; axis . tr_copy_from (& top . columns_range (irow + shift ..)) ; let (reflection_norm , not_zero) = reflection_axis_mut (& mut axis) ; axis . conjugate_mut () ; if not_zero { let refl = Reflection :: new (Unit :: new_unchecked (axis) , T :: zero ()) ; refl . reflect_rows_with_sign (& mut bottom . columns_range_mut (irow + shift ..) , & mut work . rows_range_mut (irow + 1 ..) , reflection_norm . clone () . signum () . conjugate () ,) ; top . columns_range_mut (irow + shift ..) . tr_copy_from (refl . axis ()) ; } else { top . columns_range_mut (irow + shift ..) . tr_copy_from (& axis) ; } reflection_norm }
};
}
