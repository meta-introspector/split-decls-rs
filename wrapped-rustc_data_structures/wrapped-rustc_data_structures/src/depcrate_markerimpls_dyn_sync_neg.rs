// Generated macro for impls_dyn_sync_neg (macro)
macro_rules! Depcrate_markerimpls_dyn_sync_neg {
() => {
// Module: crate::marker
// Provides: {"impls_dyn_sync_neg"}
// Dependencies: {}
macro_rules ! impls_dyn_sync_neg { ($ ([$ t1 : ty $ (where $ ($ generics1 : tt) *) ?]) *) => { $ (impl $ (<$ ($ generics1) *>) ? ! DynSync for $ t1 { }) * } ; }
};
}
