// Generated macro for impl_893 (impl)
macro_rules! Depcrate_timestampimpl_893 {
() => {
// Module: crate::timestamp
// Provides: {"impl_893"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde_core :: Serialize for Timestamp { # [inline] fn serialize < S : serde_core :: Serializer > (& self , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . collect_str (self) } }
};
}
