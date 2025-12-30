// Generated macro for offset_to (function)
macro_rules! Depcrate_stripoffset_to {
() => {
// Module: crate::strip
// Provides: {"offset_to"}
// Dependencies: {}
# [inline] fn offset_to (total : & [u8] , subslice : & [u8]) -> usize { let total = total . as_ptr () ; let subslice = subslice . as_ptr () ; debug_assert ! (total <= subslice , "`Offset::offset_to` only accepts slices of `self`") ; subslice as usize - total as usize }
};
}
