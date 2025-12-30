// Generated macro for IntoIter (struct)
macro_rules! Depcrate_list_ordered_multimapIntoIter {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator that owns and yields all key-value pairs in a multimap by cloning the keys for their possibly multiple"] # [doc = " values. This is unnecessarily expensive whenever [`Iter`] or [`IterMut`] would suit as well. The order of the"] # [doc = " yielded items is always in the order that they were inserted."] pub struct IntoIter < Key , Value > { keys : VecList < Key > , # [doc = " The iterator over the list of all values. This is ordered by time of insertion."] iter : VecListIntoIter < ValueEntry < Key , Value > > , }
};
}
