// Generated macro for impl_visitable_direct (macro)
macro_rules! Depcrate_mut_visitimpl_visitable_direct {
() => {
// Module: crate::mut_visit
// Provides: {"impl_visitable_direct"}
// Dependencies: {}
macro_rules ! impl_visitable_direct { (< mut > $ ($ ty : ty ,) *) => { $ (impl_visitable ! (|& mut self : $ ty , visitor : & mut V , _extra : () | { MutWalkable :: walk_mut (self , visitor) }) ;) * } }
};
}
