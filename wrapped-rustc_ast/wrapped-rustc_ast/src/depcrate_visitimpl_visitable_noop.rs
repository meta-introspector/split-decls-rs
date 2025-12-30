// Generated macro for impl_visitable_noop (macro)
macro_rules! Depcrate_visitimpl_visitable_noop {
() => {
// Module: crate::visit
// Provides: {"impl_visitable_noop"}
// Dependencies: {}
macro_rules ! impl_visitable_noop { (<$ lt : lifetime > $ ($ ty : ty ,) *) => { $ (impl_visitable ! (|&$ lt self : $ ty , _vis : & mut V , _extra : () | { V :: Result :: output () }) ;) * } ; }
};
}
