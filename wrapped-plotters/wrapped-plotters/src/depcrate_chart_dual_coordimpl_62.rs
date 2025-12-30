// Generated macro for impl_62 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_62 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_62"}
// Dependencies: {}
impl < DB : DrawingBackend , CT1 : ReverseCoordTranslate , CT2 : ReverseCoordTranslate > DualCoordChartContext < '_ , DB , CT1 , CT2 > { # [doc = " Convert the chart context into a pair of closures that maps the pixel coordinate into the"] # [doc = " logical coordinate for both primary coordinate system and secondary coordinate system."] pub fn into_coord_trans_pair (self ,) -> (impl Fn (BackendCoord) -> Option < CT1 :: From > , impl Fn (BackendCoord) -> Option < CT2 :: From > ,) { let coord_spec_1 = self . primary . drawing_area . into_coord_spec () ; let coord_spec_2 = self . secondary . drawing_area . into_coord_spec () ; (move | coord | coord_spec_1 . reverse_translate (coord) , move | coord | coord_spec_2 . reverse_translate (coord) ,) } }
};
}
