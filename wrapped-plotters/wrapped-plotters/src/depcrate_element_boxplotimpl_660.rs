// Generated macro for impl_660 (impl)
macro_rules! Depcrate_element_boxplotimpl_660 {
() => {
// Module: crate::element::boxplot
// Provides: {"impl_660"}
// Dependencies: {}
impl < K , V > BoxplotOrient < K , V > for BoxplotOrientV < K , V > { type XType = K ; type YType = V ; fn make_coord (key : K , val : V) -> (K , V) { (key , val) } fn with_offset (coord : BackendCoord , offset : f64) -> BackendCoord { (coord . 0 + offset as i32 , coord . 1) } }
};
}
