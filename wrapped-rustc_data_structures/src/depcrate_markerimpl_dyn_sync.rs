// Generated macro for impl_dyn_sync (macro)
macro_rules! Depcrate_markerimpl_dyn_sync {
() => {
// Module: crate::marker
// Provides: {"impl_dyn_sync"}
// Dependencies: {}
macro_rules ! impl_dyn_sync { ($ ($ ($ attr : meta) * [$ ty : ty where $ ($ generics2 : tt) *]) *) => { $ (unsafe impl <$ ($ generics2) *> DynSync for $ ty { }) * } ; }
};
}
