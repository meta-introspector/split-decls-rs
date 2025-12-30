// Generated macro for impl_visitable_direct (macro)
macro_rules! Depcrate_visitimpl_visitable_direct {
() => {
// Module: crate::visit
// Provides: {"impl_visitable_direct"}
// Dependencies: {}
macro_rules ! impl_visitable_direct { (<$ lt : lifetime > $ ($ ty : ty ,) *) => { $ (impl_visitable ! (|&$ lt self : $ ty , visitor : & mut V , _extra : () | { Walkable :: walk_ref (self , visitor) }) ;) * } ; }
};
}
