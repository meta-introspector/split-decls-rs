// Generated macro for impl_const_new (macro)
macro_rules! Depcrate_sorted_linked_listimpl_const_new {
() => {
// Module: crate::sorted_linked_list
// Provides: {"impl_const_new"}
// Dependencies: {}
macro_rules ! impl_const_new { ($ ty : ty , $ new_name : ident) => { impl < T , K , const N : usize > SortedLinkedList < T , K , N , $ ty > { # [doc = " Create a new linked list."] pub const fn $ new_name () -> Self { const { assert ! ((<$ ty >:: MAX as usize) >= (N + 1) , "The capacity is larger than `LenT` can hold, increase the size of `LenT` or reduce the capacity") ; } let mut list = SortedLinkedList { list : OwnedSortedLinkedListStorage { buffer : [const { Node { val : MaybeUninit :: uninit () , next : <$ ty >:: MAX , } } ; N] , } , head : <$ ty >:: MAX , free : 0 , phantom : PhantomData , } ; if N == 0 { list . free = <$ ty >:: MAX ; return list ; } let mut free = 0 ; while free < N - 1 { list . list . buffer [free] . next = free as $ ty + 1 ; free += 1 ; } list } } } ; }
};
}
