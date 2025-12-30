// Generated macro for impl_856 (impl)
macro_rules! Depcrate_base_indexingimpl_856 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_856"}
// Dependencies: {}
impl < 'a , T , R , C , S > MatrixIndex < 'a , T , R , C , S > for usize where T : Scalar , R : Dim , C : Dim , S : RawStorage < T , R , C > , { type Output = & 'a T ; # [doc (hidden)] # [inline (always)] fn contained_by (& self , matrix : & Matrix < T , R , C , S >) -> bool { * self < matrix . len () } # [doc (hidden)] # [inline (always)] unsafe fn get_unchecked (self , matrix : & 'a Matrix < T , R , C , S >) -> Self :: Output { let nrows = matrix . shape () . 0 ; let row = self % nrows ; let col = self / nrows ; matrix . data . get_unchecked (row , col) } }
};
}
