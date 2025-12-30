// Generated macro for BoxplotOrient (trait)
macro_rules! Depcrate_element_boxplotBoxplotOrient {
() => {
// Module: crate::element::boxplot
// Provides: {"BoxplotOrient"}
// Dependencies: {}
# [doc = " The boxplot orientation trait"] pub trait BoxplotOrient < K , V > { type XType ; type YType ; fn make_coord (key : K , val : V) -> (Self :: XType , Self :: YType) ; fn with_offset (coord : BackendCoord , offset : f64) -> BackendCoord ; }
};
}
