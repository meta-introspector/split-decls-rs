// Generated macro for impl_990 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_990 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_990"}
// Dependencies: {}
# [doc = " # Views based on index and length"] impl < T , R : Dim , C : Dim , S : RawStorage < T , R , C > > Matrix < T , R , C , S > { matrix_view_impl ! (self : & Self , MatrixView , ViewStorage , RawStorage . get_address_unchecked () , & self . data ; row , row_part , rows , rows_with_step , fixed_rows , fixed_rows_with_step , rows_generic , rows_generic_with_step , column , column_part , columns , columns_with_step , fixed_columns , fixed_columns_with_step , columns_generic , columns_generic_with_step , slice => view , slice_with_steps => view_with_steps , fixed_slice => fixed_view , fixed_slice_with_steps => fixed_view_with_steps , generic_slice => generic_view , generic_slice_with_steps => generic_view_with_steps , rows_range_pair , columns_range_pair) ; }
};
}
