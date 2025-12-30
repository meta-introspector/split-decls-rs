// Generated macro for impl_1171 (impl)
macro_rules! Depcrate_base_par_iterimpl_1171 {
() => {
// Module: crate::base::par_iter
// Provides: {"impl_1171"}
// Dependencies: {}
impl < 'a , T , R : Dim , Cols : Dim , S : RawStorage < T , R , Cols > > ParColumnIter < 'a , T , R , Cols , S > { # [doc = " Create a new parallel iterator for the given matrix."] fn new (matrix : & 'a Matrix < T , R , Cols , S >) -> Self { Self { mat : matrix } } }
};
}
