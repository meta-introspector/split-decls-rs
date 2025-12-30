// Generated macro for impl_228 (impl)
macro_rules! Depcrate_base_iterimpl_228 {
() => {
// Module: crate::base::iter
// Provides: {"impl_228"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , S : 'a + RawStorageMut < T , R , C > > DoubleEndedIterator for ColumnIterMut < 'a , T , R , C , S > { fn next_back (& mut self) -> Option < Self :: Item > { debug_assert ! (self . range . start <= self . range . end) ; if ! self . range . is_empty () { self . range . end -= 1 ; debug_assert ! (self . range . end < self . ncols ()) ; debug_assert ! (self . range . end >= self . range . start) ; Some (unsafe { (* self . mat) . column_mut (self . range . end) }) } else { None } } }
};
}
