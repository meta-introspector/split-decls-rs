// Generated macro for fpu_precision (module)
macro_rules! Depcrate_fpufpu_precision {
() => {
// Module: crate::fpu
// Provides: {"fpu_precision"}
// Dependencies: {}
# [cfg (any (not (target_arch = "x86") , target_feature = "sse2"))] mod fpu_precision { pub fn set_precision < T > () { } }
};
}
