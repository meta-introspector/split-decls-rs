// Generated macro for impl_661 (impl)
macro_rules! Depcrate_element_boxplotimpl_661 {
() => {
// Module: crate::element::boxplot
// Provides: {"impl_661"}
// Dependencies: {}
impl < K , V > BoxplotOrient < K , V > for BoxplotOrientH < K , V > { type XType = V ; type YType = K ; fn make_coord (key : K , val : V) -> (V , K) { (val , key) } fn with_offset (coord : BackendCoord , offset : f64) -> BackendCoord { (coord . 0 , coord . 1 + offset as i32) } }
};
}
