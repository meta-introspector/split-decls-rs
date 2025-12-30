// Generated macro for impl_769 (impl)
macro_rules! Depcrate_series_line_seriesimpl_769 {
() => {
// Module: crate::series::line_series
// Provides: {"impl_769"}
// Dependencies: {}
impl < I : Iterator + Clone , Size : SizeDesc , Marker : 'static > IntoIterator for DottedLineSeries < I , Size , Marker > { type Item = DottedPathElement < I , Size , Marker > ; type IntoIter = std :: iter :: Once < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { std :: iter :: once (DottedPathElement :: new (self . points , self . shift , self . spacing , self . func ,)) } }
};
}
