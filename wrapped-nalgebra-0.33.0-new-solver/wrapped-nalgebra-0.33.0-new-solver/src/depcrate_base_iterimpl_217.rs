// Generated macro for impl_217 (impl)
macro_rules! Depcrate_base_iterimpl_217 {
() => {
// Module: crate::base::iter
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorageMut < T , R , C > > Iterator for RowIterMut < 'a , T , R , C , S > { type Item = MatrixViewMut < 'a , T , U1 , C , S :: RStride , S :: CStride > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . curr < self . nrows () { let res = unsafe { (* self . mat) . row_mut (self . curr) } ; self . curr += 1 ; Some (res) } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . nrows () - self . curr , Some (self . nrows () - self . curr)) } # [inline] fn count (self) -> usize { self . nrows () - self . curr } }
};
}
