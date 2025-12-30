// Generated macro for impl_1157 (impl)
macro_rules! Depcrate_base_min_maximpl_1157 {
() => {
// Module: crate::base::min_max
// Provides: {"impl_1157"}
// Dependencies: {}
impl < T : Scalar + PartialOrd + Signed , R : Dim , C : Dim , S : RawStorage < T , R , C > > Matrix < T , R , C , S > { # [doc = " Computes the index of the matrix component with the largest absolute value."] # [doc = ""] # [doc = " # Examples:"] # [doc = ""] # [doc = " ```"] # [doc = " # use nalgebra::Matrix2x3;"] # [doc = " let mat = Matrix2x3::new(11, -12, 13,"] # [doc = "                          21, 22, -23);"] # [doc = " assert_eq!(mat.iamax_full(), (1, 2));"] # [doc = " ```"] # [inline] # [must_use] pub fn iamax_full (& self) -> (usize , usize) { assert ! (! self . is_empty () , "The input matrix must not be empty.") ; let mut the_max = unsafe { self . get_unchecked ((0 , 0)) . abs () } ; let mut the_ij = (0 , 0) ; for j in 0 .. self . ncols () { for i in 0 .. self . nrows () { let val = unsafe { self . get_unchecked ((i , j)) . abs () } ; if val > the_max { the_max = val ; the_ij = (i , j) ; } } } the_ij } }
};
}
