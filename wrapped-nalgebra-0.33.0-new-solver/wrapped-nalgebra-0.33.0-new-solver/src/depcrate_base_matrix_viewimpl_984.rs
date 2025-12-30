// Generated macro for impl_984 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_984 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_984"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S : RawStorage < T , R , C > > Matrix < T , R , C , S > { # [inline] fn assert_view_index (& self , start : (usize , usize) , shape : (usize , usize) , steps : (usize , usize) ,) { let my_shape = self . shape () ; assert ! (start . 0 + (steps . 0 + 1) * shape . 0 <= my_shape . 0 + steps . 0 , "Matrix slicing out of bounds.") ; assert ! (start . 1 + (steps . 1 + 1) * shape . 1 <= my_shape . 1 + steps . 1 , "Matrix slicing out of bounds.") ; } }
};
}
