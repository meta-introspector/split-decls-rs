// Generated macro for IterMut (struct)
macro_rules! Depcrate_list_ordered_multimapIterMut {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " An iterator that yields mutable references to all key-value pairs in a multimap. The order of the yielded items is"] # [doc = " always in the order that they were inserted."] pub struct IterMut < 'map , Key , Value > { keys : & 'map VecList < Key > , # [doc = " The iterator over the list of all values. This is ordered by time of insertion."] iter : VecListIterMut < 'map , ValueEntry < Key , Value > > , }
};
}
