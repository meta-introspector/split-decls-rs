// Generated macro for MapEntry (struct)
macro_rules! Depcrate_list_ordered_multimapMapEntry {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"MapEntry"}
// Dependencies: {}
# [doc = " The value type of the internal hash map."] # [derive (Clone)] pub (crate) struct MapEntry < Key , Value > { # [doc = " The index of the first value for this entry."] head_index : Index < ValueEntry < Key , Value > > , # [doc = " The number of values for this entry."] length : usize , # [doc = " The index of the last value for this entry."] tail_index : Index < ValueEntry < Key , Value > > , }
};
}
