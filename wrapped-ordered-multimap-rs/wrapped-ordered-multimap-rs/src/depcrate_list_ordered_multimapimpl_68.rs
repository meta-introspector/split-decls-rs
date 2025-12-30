// Generated macro for impl_68 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_68 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_68"}
// Dependencies: {}
impl < Key , Value > IntoIter < Key , Value > { # [doc = " Creates an iterator that yields immutable references to all key-value pairs in a multimap."] # [must_use] pub fn iter (& self) -> Iter < '_ , Key , Value > { Iter { keys : & self . keys , iter : self . iter . iter () , } } }
};
}
