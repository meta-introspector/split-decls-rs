// Generated macro for ConsumingIter (struct)
macro_rules! Depcrate_nodes_btreeConsumingIter {
() => {
// Module: crate::nodes::btree
// Provides: {"ConsumingIter"}
// Dependencies: {}
# [doc = " A consuming iterator over an ordered set."] pub struct ConsumingIter < A > { fwd_last : Option < A > , fwd_stack : Vec < ConsumingIterItem < A > > , back_last : Option < A > , back_stack : Vec < ConsumingIterItem < A > > , remaining : usize , }
};
}
