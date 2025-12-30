// Generated macro for impl_643 (impl)
macro_rules! Depcrate_element_errorbarimpl_643 {
() => {
// Module: crate::element::errorbar
// Provides: {"impl_643"}
// Dependencies: {}
impl < K , V > ErrorBarOrient < K , V > for ErrorBarOrientV < K , V > { type XType = K ; type YType = V ; fn make_coord (key : K , val : V) -> (K , V) { (key , val) } fn ending_coord (coord : BackendCoord , w : u32) -> (BackendCoord , BackendCoord) { ((coord . 0 - w as i32 / 2 , coord . 1) , (coord . 0 + w as i32 / 2 , coord . 1) ,) } }
};
}
