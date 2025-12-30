// Generated macro for impl_192 (impl)
macro_rules! Depcrate_civil_timeimpl_192 {
() => {
// Module: crate::civil::time
// Provides: {"impl_192"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde_core :: Serialize for Time { # [inline] fn serialize < S : serde_core :: Serializer > (& self , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . collect_str (self) } }
};
}
