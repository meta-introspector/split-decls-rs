// Generated macro for impl_226 (impl)
macro_rules! Depcrate_base_iterimpl_226 {
() => {
// Module: crate::base::iter
// Provides: {"impl_226"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorageMut < T , R , C > > Iterator for ColumnIterMut < 'a , T , R , C , S > { type Item = MatrixViewMut < 'a , T , R , U1 , S :: RStride , S :: CStride > ; # [inline] fn next (& '_ mut self) -> Option < Self :: Item > { debug_assert ! (self . range . start <= self . range . end) ; if self . range . start < self . range . end { let res = unsafe { (* self . mat) . column_mut (self . range . start) } ; self . range . start += 1 ; Some (res) } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let hint = self . range . len () ; (hint , Some (hint)) } # [inline] fn count (self) -> usize { self . range . len () } }
};
}
