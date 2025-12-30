// Generated macro for impl_38 (impl)
macro_rules! Depcrate_nodes_btreeimpl_38 {
() => {
// Module: crate::nodes::btree
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a , A : 'a + BTreeValue > Iterator for Iter < 'a , A > { type Item = & 'a A ; fn next (& mut self) -> Option < Self :: Item > { match Iter :: get (& self . fwd_path) { None => None , Some (value) => match Iter :: get (& self . back_path) { Some (last_value) if value . cmp_values (last_value) == Ordering :: Greater => None , None => None , Some (_) => { Iter :: step_forward (& mut self . fwd_path) ; self . remaining -= 1 ; Some (value) } } , } } fn size_hint (& self) -> (usize , Option < usize >) { (0 , None) } }
};
}
