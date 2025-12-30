// Generated macro for impl_766 (impl)
macro_rules! Depcrate_series_line_seriesimpl_766 {
() => {
// Module: crate::series::line_series
// Provides: {"impl_766"}
// Dependencies: {}
impl < I : Iterator + Clone , Size : SizeDesc > IntoIterator for DashedLineSeries < I , Size > { type Item = DashedPathElement < I , Size > ; type IntoIter = std :: iter :: Once < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { std :: iter :: once (DashedPathElement :: new (self . points , self . size , self . spacing , self . style ,)) } }
};
}
