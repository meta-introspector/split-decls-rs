// Generated macro for impl_44 (impl)
macro_rules! Depcrate_nodes_btreeimpl_44 {
() => {
// Module: crate::nodes::btree
// Provides: {"impl_44"}
// Dependencies: {}
impl < A > DoubleEndedIterator for ConsumingIter < A > where A : BTreeValue + Clone , { fn next_back (& mut self) -> Option < Self :: Item > { loop { match self . back_stack . pop () { None => { self . remaining = 0 ; return None ; } Some (ConsumingIterItem :: Consider (node)) => self . push_back (node) , Some (ConsumingIterItem :: Yield (value)) => { if let Some (ref last) = self . fwd_last { if value . cmp_values (last) != Ordering :: Greater { self . fwd_stack . clear () ; self . back_stack . clear () ; self . remaining = 0 ; return None ; } } self . remaining -= 1 ; self . back_last = Some (value . clone ()) ; return Some (value) ; } } } } }
};
}
