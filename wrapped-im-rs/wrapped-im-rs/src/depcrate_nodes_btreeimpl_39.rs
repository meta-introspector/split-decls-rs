// Generated macro for impl_39 (impl)
macro_rules! Depcrate_nodes_btreeimpl_39 {
() => {
// Module: crate::nodes::btree
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a , A : 'a + BTreeValue > DoubleEndedIterator for Iter < 'a , A > { fn next_back (& mut self) -> Option < Self :: Item > { match Iter :: get (& self . back_path) { None => None , Some (value) => match Iter :: get (& self . fwd_path) { Some (last_value) if value . cmp_values (last_value) == Ordering :: Less => None , None => None , Some (_) => { Iter :: step_back (& mut self . back_path) ; self . remaining -= 1 ; Some (value) } } , } } }
};
}
