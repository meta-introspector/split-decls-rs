// Generated macro for portable (module)
macro_rules! Depcrateportable {
() => {
// Module: crate
// Provides: {"portable"}
// Dependencies: {}
# [cfg (any (not (any (target_arch = "aarch64" , target_arch = "x86" , target_arch = "x86_64")) , miri))] mod portable ;
};
}
