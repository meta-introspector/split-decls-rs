// Generated macro for impl_118 (impl)
macro_rules! Depcrate_defimpl_118 {
() => {
// Module: crate::def
// Provides: {"impl_118"}
// Dependencies: {}
impl < T > PerNS < Option < T > > { # [doc = " Returns `true` if all the items in this collection are `None`."] pub fn is_empty (& self) -> bool { self . type_ns . is_none () && self . value_ns . is_none () && self . macro_ns . is_none () } # [doc = " Returns an iterator over the items which are `Some`."] # [doc = ""] # [doc = " Note: Do you really want to use this? Often you know which namespace a"] # [doc = " name will belong in, and you can consider just that namespace directly,"] # [doc = " rather than iterating through all of them."] pub fn present_items (self) -> impl Iterator < Item = T > { [self . type_ns , self . value_ns , self . macro_ns] . into_iter () . flatten () } }
};
}
