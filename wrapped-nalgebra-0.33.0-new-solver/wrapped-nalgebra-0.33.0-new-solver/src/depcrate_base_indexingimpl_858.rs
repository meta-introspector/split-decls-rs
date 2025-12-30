// Generated macro for impl_858 (impl)
macro_rules! Depcrate_base_indexingimpl_858 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_858"}
// Dependencies: {}
impl < 'a , T : 'a , R , C , S > MatrixIndex < 'a , T , R , C , S > for (usize , usize) where R : Dim , C : Dim , S : RawStorage < T , R , C > , { type Output = & 'a T ; # [doc (hidden)] # [inline (always)] fn contained_by (& self , matrix : & Matrix < T , R , C , S >) -> bool { let (rows , cols) = self ; let (nrows , ncols) = matrix . shape_generic () ; DimRange :: contained_by (rows , nrows) && DimRange :: contained_by (cols , ncols) } # [doc (hidden)] # [inline (always)] unsafe fn get_unchecked (self , matrix : & 'a Matrix < T , R , C , S >) -> Self :: Output { let (row , col) = self ; matrix . data . get_unchecked (row , col) } }
};
}
