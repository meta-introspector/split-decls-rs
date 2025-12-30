// Generated macro for impl_220 (impl)
macro_rules! Depcrate_base_iterimpl_220 {
() => {
// Module: crate::base::iter
// Provides: {"impl_220"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorage < T , R , C > > ColumnIter < 'a , T , R , C , S > { # [doc = " a new column iterator covering all columns of the matrix"] pub (crate) fn new (mat : & 'a Matrix < T , R , C , S >) -> Self { ColumnIter { mat , range : 0 .. mat . ncols () , } } # [cfg (feature = "rayon")] pub (crate) fn split_at (self , index : usize) -> (Self , Self) { let split_pos = (self . range . start + index) . min (self . range . end) ; let left_iter = ColumnIter { mat : self . mat , range : self . range . start .. split_pos , } ; let right_iter = ColumnIter { mat : self . mat , range : split_pos .. self . range . end , } ; (left_iter , right_iter) } }
};
}
