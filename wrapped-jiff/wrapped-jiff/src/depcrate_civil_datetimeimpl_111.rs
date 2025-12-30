// Generated macro for impl_111 (impl)
macro_rules! Depcrate_civil_datetimeimpl_111 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_111"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde_core :: Serialize for DateTime { # [inline] fn serialize < S : serde_core :: Serializer > (& self , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . collect_str (self) } }
};
}
