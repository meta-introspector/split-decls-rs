// Generated macro for Node (struct)
macro_rules! Depcrate_sorted_linked_listNode {
() => {
// Module: crate::sorted_linked_list
// Provides: {"Node"}
// Dependencies: {}
# [doc = " A node in the [`SortedLinkedList`]."] # [cfg_attr (feature = "zeroize" , derive (Zeroize))] pub struct Node < T , Idx > { val : MaybeUninit < T > , next : Idx , }
};
}
