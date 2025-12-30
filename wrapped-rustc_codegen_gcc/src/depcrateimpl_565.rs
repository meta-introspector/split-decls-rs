// Generated macro for impl_565 (impl)
macro_rules! Depcrateimpl_565 {
() => {
// Module: crate
// Provides: {"impl_565"}
// Dependencies: {}
impl LockedTargetInfo { fn cpu_supports (& self , feature : & str) -> bool { self . info . lock () . expect ("lock") . cpu_supports (feature) } fn supports_target_dependent_type (& self , typ : CType) -> bool { self . info . lock () . expect ("lock") . supports_target_dependent_type (typ) } }
};
}
