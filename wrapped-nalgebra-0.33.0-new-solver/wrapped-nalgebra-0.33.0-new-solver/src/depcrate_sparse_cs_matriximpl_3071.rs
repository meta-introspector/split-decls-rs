// Generated macro for impl_3071 (impl)
macro_rules! Depcrate_sparse_cs_matriximpl_3071 {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"impl_3071"}
// Dependencies: {}
impl < T : Scalar , R : Dim , C : Dim > CsMatrix < T , R , C > where DefaultAllocator : Allocator < C > , { # [doc = " Creates a new compressed sparse column matrix with the specified dimension and"] # [doc = " `nvals` possible non-zero values."] pub fn new_uninitialized_generic (nrows : R , ncols : C , nvals : usize) -> Self { let mut i = Vec :: with_capacity (nvals) ; unsafe { i . set_len (nvals) ; } i . shrink_to_fit () ; let mut vals = Vec :: with_capacity (nvals) ; unsafe { vals . set_len (nvals) ; } vals . shrink_to_fit () ; CsMatrix { data : CsVecStorage { shape : (nrows , ncols) , p : OVector :: zeros_generic (ncols , Const :: < 1 >) , i , vals , } , _phantoms : PhantomData , } } }
};
}
