// Generated macro for MatrixIndex (trait)
macro_rules! Depcrate_base_indexingMatrixIndex {
() => {
// Module: crate::base::indexing
// Provides: {"MatrixIndex"}
// Dependencies: {}
# [doc = " A helper trait used for indexing operations."] pub trait MatrixIndex < 'a , T , R : Dim , C : Dim , S : RawStorage < T , R , C > > : Sized { # [doc = " The output type returned by methods."] type Output : 'a ; # [doc = " Produces true if the given matrix is contained by this index."] # [doc (hidden)] fn contained_by (& self , matrix : & Matrix < T , R , C , S >) -> bool ; # [doc = " Produces a shared view of the data at this location if in bounds,"] # [doc = " or `None`, otherwise."] # [doc (hidden)] # [inline (always)] fn get (self , matrix : & 'a Matrix < T , R , C , S >) -> Option < Self :: Output > { if self . contained_by (matrix) { Some (unsafe { self . get_unchecked (matrix) }) } else { None } } # [doc = " Produces a shared view of the data at this location if in bounds"] # [doc = " without any bounds checking."] # [doc (hidden)] unsafe fn get_unchecked (self , matrix : & 'a Matrix < T , R , C , S >) -> Self :: Output ; # [doc = " Produces a shared view to the data at this location, or panics"] # [doc = " if out of bounds."] # [doc (hidden)] # [inline (always)] fn index (self , matrix : & 'a Matrix < T , R , C , S >) -> Self :: Output { self . get (matrix) . expect ("Index out of bounds.") } }
};
}
