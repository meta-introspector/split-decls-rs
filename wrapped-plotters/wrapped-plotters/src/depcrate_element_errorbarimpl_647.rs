// Generated macro for impl_647 (impl)
macro_rules! Depcrate_element_errorbarimpl_647 {
() => {
// Module: crate::element::errorbar
// Provides: {"impl_647"}
// Dependencies: {}
impl < 'a , K : Clone , V : Clone , O : ErrorBarOrient < K , V > > PointCollection < 'a , (O :: XType , O :: YType) > for & 'a ErrorBar < K , V , O > { type Point = (O :: XType , O :: YType) ; type IntoIter = Vec < Self :: Point > ; fn point_iter (self) -> Self :: IntoIter { self . values . iter () . map (| v | O :: make_coord (self . key . clone () , v . clone ())) . collect () } }
};
}
