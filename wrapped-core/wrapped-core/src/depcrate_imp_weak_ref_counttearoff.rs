// Generated macro for TearOff (struct)
macro_rules! Depcrate_imp_weak_ref_countTearOff {
() => {
// Module: crate::imp::weak_ref_count
// Provides: {"TearOff"}
// Dependencies: {}
# [repr (C)] struct TearOff { strong_vtable : * const IWeakReferenceSource_Vtbl , weak_vtable : * const IWeakReference_Vtbl , object : * mut c_void , strong_count : RefCount , weak_count : RefCount , }
};
}
