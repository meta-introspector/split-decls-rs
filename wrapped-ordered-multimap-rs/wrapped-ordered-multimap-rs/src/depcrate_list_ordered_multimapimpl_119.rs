// Generated macro for impl_119 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_119 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_119"}
// Dependencies: {}
impl < Key , Value > ValuesMut < '_ , Key , Value > { # [doc = " Creates an iterator that yields immutable references to all values of a multimap."] # [must_use] pub fn iter (& self) -> Values < '_ , Key , Value > { Values (self . 0 . iter ()) } }
};
}
