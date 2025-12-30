// Generated macro for impl_pod (macro)
macro_rules! Depcrate_slice_blockedimpl_pod {
() => {
// Module: crate::slice::blocked
// Provides: {"impl_pod"}
// Dependencies: {}
macro_rules ! impl_pod { (@ array $ ($ e : expr) ,+) => { $ (unsafe impl < T > Block for [T ; $ e] { type Item = T ; # [inline (always)] fn capacity () -> usize { $ e } }) + } ; ($ ($ t : ty) +) => { $ (unsafe impl Block for $ t { }) + } ; }
};
}
