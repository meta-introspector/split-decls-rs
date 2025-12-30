// Generated macro for impl_222 (impl)
macro_rules! Depcrate_base_iterimpl_222 {
() => {
// Module: crate::base::iter
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorage < T , R , C > > DoubleEndedIterator for ColumnIter < 'a , T , R , C , S > { fn next_back (& mut self) -> Option < Self :: Item > { debug_assert ! (self . range . start <= self . range . end) ; if ! self . range . is_empty () { self . range . end -= 1 ; debug_assert ! (self . range . end < self . mat . ncols ()) ; debug_assert ! (self . range . end >= self . range . start) ; Some (self . mat . column (self . range . end)) } else { None } } }
};
}
