// Generated macro for Iter (struct)
macro_rules! Depcrate_list_ordered_multimapIter {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator that yields immutable references to all key-value pairs in a multimap. The order of the yielded items is"] # [doc = " always in the order that they were inserted."] pub struct Iter < 'map , Key , Value > { keys : & 'map VecList < Key > , # [doc = " The iterator over the list of all values. This is ordered by time of insertion."] iter : VecListIter < 'map , ValueEntry < Key , Value > > , }
};
}
