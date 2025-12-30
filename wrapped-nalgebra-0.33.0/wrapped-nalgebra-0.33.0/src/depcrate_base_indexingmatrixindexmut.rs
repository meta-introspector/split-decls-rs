// Generated macro for MatrixIndexMut (trait)
macro_rules! Depcrate_base_indexingMatrixIndexMut {
() => {
// Module: crate::base::indexing
// Provides: {"MatrixIndexMut"}
// Dependencies: {}
# [doc = " A helper trait used for indexing operations."] pub trait MatrixIndexMut < 'a , T , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > : MatrixIndex < 'a , T , R , C , S > { # [doc = " The output type returned by methods."] type OutputMut : 'a ; # [doc = " Produces a mutable view of the data at this location, without"] # [doc = " performing any bounds checking."] # [doc (hidden)] unsafe fn get_unchecked_mut (self , matrix : & 'a mut Matrix < T , R , C , S >) -> Self :: OutputMut ; # [doc = " Produces a mutable view of the data at this location, if in"] # [doc = " bounds."] # [doc (hidden)] # [inline (always)] fn get_mut (self , matrix : & 'a mut Matrix < T , R , C , S >) -> Option < Self :: OutputMut > { if self . contained_by (matrix) { Some (unsafe { self . get_unchecked_mut (matrix) }) } else { None } } # [doc = " Produces a mutable view of the data at this location, or panics"] # [doc = " if out of bounds."] # [doc (hidden)] # [inline (always)] fn index_mut (self , matrix : & 'a mut Matrix < T , R , C , S >) -> Self :: OutputMut { self . get_mut (matrix) . expect ("Index out of bounds.") } }
};
}
