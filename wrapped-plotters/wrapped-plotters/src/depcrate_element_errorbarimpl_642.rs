// Generated macro for impl_642 (impl)
macro_rules! Depcrate_element_errorbarimpl_642 {
() => {
// Module: crate::element::errorbar
// Provides: {"impl_642"}
// Dependencies: {}
impl < K , V > ErrorBarOrient < K , V > for ErrorBarOrientH < K , V > { type XType = V ; type YType = K ; fn make_coord (key : K , val : V) -> (V , K) { (val , key) } fn ending_coord (coord : BackendCoord , w : u32) -> (BackendCoord , BackendCoord) { ((coord . 0 , coord . 1 - w as i32 / 2) , (coord . 0 , coord . 1 + w as i32 / 2) ,) } }
};
}
