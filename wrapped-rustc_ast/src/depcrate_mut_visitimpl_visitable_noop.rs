// Generated macro for impl_visitable_noop (macro)
macro_rules! Depcrate_mut_visitimpl_visitable_noop {
() => {
// Module: crate::mut_visit
// Provides: {"impl_visitable_noop"}
// Dependencies: {}
macro_rules ! impl_visitable_noop { (< mut > $ ($ ty : ty ,) *) => { $ (impl_visitable ! (|& mut self : $ ty , _vis : & mut V , _extra : () | { }) ;) * } ; }
};
}
