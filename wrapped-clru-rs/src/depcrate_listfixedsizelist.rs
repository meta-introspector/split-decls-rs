// Generated macro for FixedSizeList (struct)
macro_rules! Depcrate_listFixedSizeList {
() => {
// Module: crate::list
// Provides: {"FixedSizeList"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct FixedSizeList < T > { capacity : usize , nodes : Vec < Option < FixedSizeListNode < T > > > , free : Vec < usize > , front : usize , back : usize , }
};
}
