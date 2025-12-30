// Generated macro for impl_765 (impl)
macro_rules! Depcrate_spanimpl_765 {
() => {
// Module: crate::span
// Provides: {"impl_765"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde_core :: Serialize for Span { # [inline] fn serialize < S : serde_core :: Serializer > (& self , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . collect_str (self) } }
};
}
