// Generated macro for impl_112 (impl)
macro_rules! Depcrate_endianityimpl_112 {
() => {
// Module: crate::endianity
// Provides: {"impl_112"}
// Dependencies: {}
impl Default for RunTimeEndian { # [cfg (target_endian = "little")] # [inline] fn default () -> RunTimeEndian { RunTimeEndian :: Little } # [cfg (target_endian = "big")] # [inline] fn default () -> RunTimeEndian { RunTimeEndian :: Big } }
};
}
