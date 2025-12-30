// Generated macro for impl_51 (impl)
macro_rules! Depcrate_civil_dateimpl_51 {
() => {
// Module: crate::civil::date
// Provides: {"impl_51"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde_core :: Serialize for Date { # [inline] fn serialize < S : serde_core :: Serializer > (& self , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . collect_str (self) } }
};
}
