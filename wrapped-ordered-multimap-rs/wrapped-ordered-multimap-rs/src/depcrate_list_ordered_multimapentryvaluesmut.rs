// Generated macro for EntryValuesMut (struct)
macro_rules! Depcrate_list_ordered_multimapEntryValuesMut {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"EntryValuesMut"}
// Dependencies: {}
# [doc = " An iterator that yields mutable references to all values of a given key. The order of the values is always in the"] # [doc = " order that they were inserted."] pub struct EntryValuesMut < 'map , Key , Value > { # [doc = " The first index of the values not yet yielded."] head_index : Option < Index < ValueEntry < Key , Value > > > , # [doc = " Because [`EntryValuesMut::values`] is a pointer, we need to have a phantom data here for the lifetime parameter."] phantom : PhantomData < & 'map mut VecList < ValueEntry < Key , Value > > > , # [doc = " The remaining number of values to be yielded."] remaining : usize , # [doc = " The last index of the values not yet yielded."] tail_index : Option < Index < ValueEntry < Key , Value > > > , # [doc = " The list of the values in the map. This is ordered by time of insertion."] values : * mut VecList < ValueEntry < Key , Value > > , }
};
}
