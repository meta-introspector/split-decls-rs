// Generated macro for impl_43 (impl)
macro_rules! Depcrate_nodes_btreeimpl_43 {
() => {
// Module: crate::nodes::btree
// Provides: {"impl_43"}
// Dependencies: {}
impl < A > Iterator for ConsumingIter < A > where A : BTreeValue + Clone , { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { loop { match self . fwd_stack . pop () { None => { self . remaining = 0 ; return None ; } Some (ConsumingIterItem :: Consider (node)) => self . push_fwd (node) , Some (ConsumingIterItem :: Yield (value)) => { if let Some (ref last) = self . back_last { if value . cmp_values (last) != Ordering :: Less { self . fwd_stack . clear () ; self . back_stack . clear () ; self . remaining = 0 ; return None ; } } self . remaining -= 1 ; self . fwd_last = Some (value . clone ()) ; return Some (value) ; } } } } fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
