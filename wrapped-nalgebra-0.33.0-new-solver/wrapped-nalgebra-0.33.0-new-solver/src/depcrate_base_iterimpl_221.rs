// Generated macro for impl_221 (impl)
macro_rules! Depcrate_base_iterimpl_221 {
() => {
// Module: crate::base::iter
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorage < T , R , C > > Iterator for ColumnIter < 'a , T , R , C , S > { type Item = MatrixView < 'a , T , R , U1 , S :: RStride , S :: CStride > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { debug_assert ! (self . range . start <= self . range . end) ; if self . range . start < self . range . end { let res = self . mat . column (self . range . start) ; self . range . start += 1 ; Some (res) } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let hint = self . range . len () ; (hint , Some (hint)) } # [inline] fn count (self) -> usize { self . range . len () } }
};
}
