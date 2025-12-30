// Generated macro for impl_527 (impl)
macro_rules! Depcrate_sorted_linked_listimpl_527 {
() => {
// Module: crate::sorted_linked_list
// Provides: {"impl_527"}
// Dependencies: {}
impl < T , Idx , K , S > fmt :: Debug for SortedLinkedListInner < T , Idx , K , S > where T : Ord + core :: fmt :: Debug , Idx : LenType , K : Kind , S : ? Sized + SortedLinkedListStorage < T , Idx > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
