// Generated macro for impl_1407 (impl)
macro_rules! Depcrate_zonedimpl_1407 {
() => {
// Module: crate::zoned
// Provides: {"impl_1407"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde_core :: Serialize for Zoned { # [inline] fn serialize < S : serde_core :: Serializer > (& self , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . collect_str (self) } }
};
}
