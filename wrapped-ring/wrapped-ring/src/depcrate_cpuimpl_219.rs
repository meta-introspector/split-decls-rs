// Generated macro for impl_219 (impl)
macro_rules! Depcrate_cpuimpl_219 {
() => {
// Module: crate::cpu
// Provides: {"impl_219"}
// Dependencies: {}
impl < F > GetFeature < F > for Features where features :: Values : GetFeature < F > , { # [inline (always)] fn get_feature (& self) -> Option < F > { self . values () . get_feature () } }
};
}
