// Generated macro for ValuesMut (struct)
macro_rules! Depcrate_list_ordered_multimapValuesMut {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"ValuesMut"}
// Dependencies: {}
# [doc = " An iterator that yields mutable references to all values of a multimap. The order of the values is always in the"] # [doc = " order that they were inserted."] pub struct ValuesMut < 'map , Key , Value > (VecListIterMut < 'map , ValueEntry < Key , Value > >) ;
};
}
