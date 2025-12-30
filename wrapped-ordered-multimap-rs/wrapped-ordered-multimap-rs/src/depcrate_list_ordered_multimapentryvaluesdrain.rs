// Generated macro for EntryValuesDrain (struct)
macro_rules! Depcrate_list_ordered_multimapEntryValuesDrain {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"EntryValuesDrain"}
// Dependencies: {}
# [doc = " An iterator that moves all values of a given key out of a multimap but preserves the underlying capacity. The order"] # [doc = " of the values is always in the order that they were inserted."] pub struct EntryValuesDrain < 'map , Key , Value > { # [doc = " The first index of the values not yet yielded."] head_index : Option < Index < ValueEntry < Key , Value > > > , # [doc = " The remaining number of values to be yielded."] remaining : usize , # [doc = " The last index of the values not yet yielded."] tail_index : Option < Index < ValueEntry < Key , Value > > > , # [doc = " The list of the values in the map. This is ordered by time of insertion."] values : & 'map mut VecList < ValueEntry < Key , Value > > , }
};
}
