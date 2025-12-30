// Generated macro for impl_9 (impl)
macro_rules! Depcrate_comboimpl_9 {
() => {
// Module: crate::combo
// Provides: {"impl_9"}
// Dependencies: {}
impl < DT , Z > DateTimeMarkers for Combo < DT , Z > where DT : DateTimeMarkers , Z : DateTimeMarkers , { type D = DT :: D ; type T = DT :: T ; type Z = Z :: Z ; type GluePatternV1 = datetime_marker_helper ! (@ glue , yes) ; }
};
}
