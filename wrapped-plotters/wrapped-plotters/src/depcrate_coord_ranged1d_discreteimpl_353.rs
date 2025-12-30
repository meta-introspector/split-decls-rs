// Generated macro for impl_353 (impl)
macro_rules! Depcrate_coord_ranged1d_discreteimpl_353 {
() => {
// Module: crate::coord::ranged1d::discrete
// Provides: {"impl_353"}
// Dependencies: {}
impl < D : DiscreteRanged > Ranged for SegmentedCoord < D > { type FormatOption = NoDefaultFormatting ; type ValueType = SegmentValue < D :: ValueType > ; fn map (& self , value : & Self :: ValueType , limit : (i32 , i32)) -> i32 { let margin = ((limit . 1 - limit . 0) as f32 / self . 0 . size () as f32) . round () as i32 ; match value { SegmentValue :: Exact (coord) => self . 0 . map (coord , (limit . 0 , limit . 1 - margin)) , SegmentValue :: CenterOf (coord) => { let left = self . 0 . map (coord , (limit . 0 , limit . 1 - margin)) ; if let Some (idx) = self . 0 . index_of (coord) { if idx + 1 < self . 0 . size () { let right = self . 0 . map (& self . 0 . from_index (idx + 1) . unwrap () , (limit . 0 , limit . 1 - margin) ,) ; return (left + right) / 2 ; } } left + margin / 2 } SegmentValue :: Last => limit . 1 , } } fn key_points < HintType : KeyPointHint > (& self , hint : HintType) -> Vec < Self :: ValueType > { self . 0 . key_points (hint) . into_iter () . map (SegmentValue :: CenterOf) . collect () } fn range (& self) -> Range < Self :: ValueType > { let range = self . 0 . range () ; SegmentValue :: Exact (range . start) .. SegmentValue :: Exact (range . end) } }
};
}
