// Generated macro for impl_213 (impl)
macro_rules! Depcrate_base_iterimpl_213 {
() => {
// Module: crate::base::iter
// Provides: {"impl_213"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorage < T , R , C > > Iterator for RowIter < 'a , T , R , C , S > { type Item = MatrixView < 'a , T , U1 , C , S :: RStride , S :: CStride > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . curr < self . mat . nrows () { let res = self . mat . row (self . curr) ; self . curr += 1 ; Some (res) } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . mat . nrows () - self . curr , Some (self . mat . nrows () - self . curr) ,) } # [inline] fn count (self) -> usize { self . mat . nrows () - self . curr } }
};
}
