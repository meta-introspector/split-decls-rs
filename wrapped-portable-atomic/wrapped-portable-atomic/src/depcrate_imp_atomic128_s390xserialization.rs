// Generated macro for serialization (macro)
macro_rules! Depcrate_imp_atomic128_s390xserialization {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"serialization"}
// Dependencies: {}
# [cfg (not (any (target_feature = "fast-serialization" , portable_atomic_target_feature = "fast-serialization" ,)))] macro_rules ! serialization { () => { "bcr 15, 0" } ; }
};
}
