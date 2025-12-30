// Generated macro for impl_dyn_send (macro)
macro_rules! Depcrate_markerimpl_dyn_send {
() => {
// Module: crate::marker
// Provides: {"impl_dyn_send"}
// Dependencies: {}
macro_rules ! impl_dyn_send { ($ ($ ($ attr : meta) * [$ ty : ty where $ ($ generics2 : tt) *]) *) => { $ (unsafe impl <$ ($ generics2) *> DynSend for $ ty { }) * } ; }
};
}
