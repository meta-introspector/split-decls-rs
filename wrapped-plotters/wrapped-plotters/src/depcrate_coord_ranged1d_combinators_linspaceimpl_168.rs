// Generated macro for impl_168 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_linspaceimpl_168 {
() => {
// Module: crate::coord::ranged1d::combinators::linspace
// Provides: {"impl_168"}
// Dependencies: {}
impl < T : Ranged , S : Clone , R : LinspaceRoundingMethod < T :: ValueType > > Ranged for Linspace < T , S , R > where T :: ValueType : Add < S , Output = T :: ValueType > + PartialOrd + Clone , { type FormatOption = NoDefaultFormatting ; type ValueType = T :: ValueType ; fn range (& self) -> Range < T :: ValueType > { self . inner . range () } fn map (& self , value : & T :: ValueType , limit : (i32 , i32)) -> i32 { self . inner . map (value , limit) } fn key_points < Hint : KeyPointHint > (& self , hint : Hint) -> Vec < T :: ValueType > { if self . grid_value . is_empty () { return vec ! [] ; } let idx_range : RangedCoordusize = (0 .. (self . grid_value . len () - 1)) . into () ; idx_range . key_points (hint) . into_iter () . map (| x | self . grid_value [x] . clone ()) . collect () } }
};
}
