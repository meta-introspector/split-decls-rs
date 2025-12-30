// Generated macro for Values (struct)
macro_rules! Depcrate_list_ordered_multimapValues {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"Values"}
// Dependencies: {}
# [doc = " An iterator that yields immutable references to all values of a multimap. The order of the values is always in the"] # [doc = " order that they were inserted."] pub struct Values < 'map , Key , Value > (VecListIter < 'map , ValueEntry < Key , Value > >) ;
};
}
