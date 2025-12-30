// Generated macro for impl_212 (impl)
macro_rules! Depcrate_base_iterimpl_212 {
() => {
// Module: crate::base::iter
// Provides: {"impl_212"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorage < T , R , C > > RowIter < 'a , T , R , C , S > { pub (crate) fn new (mat : & 'a Matrix < T , R , C , S >) -> Self { RowIter { mat , curr : 0 } } }
};
}
