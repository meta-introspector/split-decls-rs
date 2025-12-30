// Generated macro for impl_pod (macro)
macro_rules! Depcrate_sliceimpl_pod {
() => {
// Module: crate::slice
// Provides: {"impl_pod"}
// Dependencies: {}
macro_rules ! impl_pod { (@ array $ ($ e : expr) ,+) => { $ (unsafe impl < T > Pod for [T ; $ e] where T : Pod { }) + } ; ($ ($ t : ty) +) => { $ (unsafe impl Pod for $ t { }) + } ; }
};
}
