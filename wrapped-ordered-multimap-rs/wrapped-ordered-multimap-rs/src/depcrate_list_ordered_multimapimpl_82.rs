// Generated macro for impl_82 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_82 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_82"}
// Dependencies: {}
impl < Key , Value > IterMut < '_ , Key , Value > { # [doc = " Creates an iterator that yields immutable references to all key-value pairs in a multimap."] # [must_use] pub fn iter (& self) -> Iter < '_ , Key , Value > { Iter { keys : self . keys , iter : self . iter . iter () , } } }
};
}
