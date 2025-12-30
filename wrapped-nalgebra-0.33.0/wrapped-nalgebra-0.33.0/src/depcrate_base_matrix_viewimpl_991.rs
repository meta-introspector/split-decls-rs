// Generated macro for impl_991 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_991 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_991"}
// Dependencies: {}
# [doc = " # Mutable views based on index and length"] impl < T , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > Matrix < T , R , C , S > { matrix_view_impl ! (self : & mut Self , MatrixViewMut , ViewStorageMut , RawStorageMut . get_address_unchecked_mut () , & mut self . data ; row_mut , row_part_mut , rows_mut , rows_with_step_mut , fixed_rows_mut , fixed_rows_with_step_mut , rows_generic_mut , rows_generic_with_step_mut , column_mut , column_part_mut , columns_mut , columns_with_step_mut , fixed_columns_mut , fixed_columns_with_step_mut , columns_generic_mut , columns_generic_with_step_mut , slice_mut => view_mut , slice_with_steps_mut => view_with_steps_mut , fixed_slice_mut => fixed_view_mut , fixed_slice_with_steps_mut => fixed_view_with_steps_mut , generic_slice_mut => generic_view_mut , generic_slice_with_steps_mut => generic_view_with_steps_mut , rows_range_pair_mut , columns_range_pair_mut) ; }
};
}
