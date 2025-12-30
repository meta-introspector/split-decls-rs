// Generated macro for impl_667 (impl)
macro_rules! Depcrate_element_boxplotimpl_667 {
() => {
// Module: crate::element::boxplot
// Provides: {"impl_667"}
// Dependencies: {}
impl < 'a , K : Clone , O : BoxplotOrient < K , f32 > > PointCollection < 'a , (O :: XType , O :: YType) > for & 'a Boxplot < K , O > { type Point = (O :: XType , O :: YType) ; type IntoIter = Vec < Self :: Point > ; fn point_iter (self) -> Self :: IntoIter { self . values . iter () . map (| v | O :: make_coord (self . key . clone () , * v)) . collect () } }
};
}
