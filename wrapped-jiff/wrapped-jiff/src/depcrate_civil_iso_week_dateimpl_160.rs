// Generated macro for impl_160 (impl)
macro_rules! Depcrate_civil_iso_week_dateimpl_160 {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"impl_160"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde_core :: Serialize for ISOWeekDate { # [inline] fn serialize < S : serde_core :: Serializer > (& self , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . collect_str (self) } }
};
}
