// Generated macro for impl_135 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_ckpsimpl_135 {
() => {
// Module: crate::coord::ranged1d::combinators::ckps
// Provides: {"impl_135"}
// Dependencies: {}
impl < R : Ranged > Ranged for WithKeyPointMethod < R > { type ValueType = R :: ValueType ; type FormatOption = R :: FormatOption ; fn range (& self) -> Range < Self :: ValueType > { self . inner . range () } fn map (& self , value : & Self :: ValueType , limit : (i32 , i32)) -> i32 { self . inner . map (value , limit) } fn key_points < Hint : KeyPointHint > (& self , hint : Hint) -> Vec < Self :: ValueType > { if hint . weight () . allow_light_points () { (self . light_func) (hint . max_num_points ()) } else { (self . bold_func) (hint . max_num_points ()) } } fn axis_pixel_range (& self , limit : (i32 , i32)) -> Range < i32 > { self . inner . axis_pixel_range (limit) } }
};
}
