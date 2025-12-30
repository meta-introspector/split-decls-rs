// Generated macro for neon (module)
macro_rules! Depcrateneon {
() => {
// Module: crate
// Provides: {"neon"}
// Dependencies: {}
# [cfg (all (target_arch = "aarch64" , target_feature = "neon" , not (miri)))] mod neon ;
};
}
