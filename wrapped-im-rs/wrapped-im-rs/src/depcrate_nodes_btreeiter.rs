// Generated macro for Iter (struct)
macro_rules! Depcrate_nodes_btreeIter {
() => {
// Module: crate::nodes::btree
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over an ordered set."] pub struct Iter < 'a , A > { fwd_path : Vec < (& 'a Node < A > , usize) > , back_path : Vec < (& 'a Node < A > , usize) > , pub (crate) remaining : usize , }
};
}
