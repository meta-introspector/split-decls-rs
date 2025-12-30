// Generated macro for impl_49 (impl)
macro_rules! Depcrate_nodes_btreeimpl_49 {
() => {
// Module: crate::nodes::btree
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a , A : 'a > DiffIter < 'a , A > { pub (crate) fn new (old : & 'a Node < A > , new : & 'a Node < A >) -> Self { DiffIter { old_stack : if old . keys . is_empty () { Vec :: new () } else { vec ! [IterItem :: Consider (old)] } , new_stack : if new . keys . is_empty () { Vec :: new () } else { vec ! [IterItem :: Consider (new)] } , } } fn push_node (stack : & mut Vec < IterItem < 'a , A > > , maybe_node : & 'a Option < PoolRef < Node < A > > >) { if let Some (ref node) = * maybe_node { stack . push (IterItem :: Consider (node)) } } fn push (stack : & mut Vec < IterItem < 'a , A > > , node : & 'a Node < A >) { for n in 0 .. node . keys . len () { let i = node . keys . len () - n ; Self :: push_node (stack , & node . children [i]) ; stack . push (IterItem :: Yield (& node . keys [i - 1])) ; } Self :: push_node (stack , & node . children [0]) ; } }
};
}
