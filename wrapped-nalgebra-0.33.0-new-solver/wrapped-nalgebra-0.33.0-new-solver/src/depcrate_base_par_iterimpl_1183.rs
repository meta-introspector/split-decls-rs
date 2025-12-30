// Generated macro for impl_1183 (impl)
macro_rules! Depcrate_base_par_iterimpl_1183 {
() => {
// Module: crate::base::par_iter
// Provides: {"impl_1183"}
// Dependencies: {}
# [doc = " this implementation is safe because we are enforcing exclusive access"] # [doc = " to the columns through the active range of the iterator"] unsafe impl < 'a , T : Scalar , R : Dim , C : Dim , S : 'a + RawStorageMut < T , R , C > > Send for ColumnIterMut < 'a , T , R , C , S > { }
};
}
