// Generated macro for impl_732 (impl)
macro_rules! Depcrate_signed_durationimpl_732 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_732"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde_core :: Serialize for SignedDuration { # [inline] fn serialize < S : serde_core :: Serializer > (& self , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . collect_str (self) } }
};
}
