// Generated macro for impl_216 (impl)
macro_rules! Depcrate_base_iterimpl_216 {
() => {
// Module: crate::base::iter
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorageMut < T , R , C > > RowIterMut < 'a , T , R , C , S > { pub (crate) fn new (mat : & 'a mut Matrix < T , R , C , S >) -> Self { RowIterMut { mat , curr : 0 , phantom : PhantomData , } } fn nrows (& self) -> usize { unsafe { (* self . mat) . nrows () } } }
};
}
