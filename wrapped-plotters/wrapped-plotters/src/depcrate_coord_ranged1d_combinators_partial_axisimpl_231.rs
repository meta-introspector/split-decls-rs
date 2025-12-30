// Generated macro for impl_231 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_partial_axisimpl_231 {
() => {
// Module: crate::coord::ranged1d::combinators::partial_axis
// Provides: {"impl_231"}
// Dependencies: {}
impl < R : Ranged > Ranged for PartialAxis < R > where R :: ValueType : Clone , { type FormatOption = DefaultFormatting ; type ValueType = R :: ValueType ; fn map (& self , value : & Self :: ValueType , limit : (i32 , i32)) -> i32 { self . 0 . map (value , limit) } fn key_points < Hint : KeyPointHint > (& self , hint : Hint) -> Vec < Self :: ValueType > { self . 0 . key_points (hint) } fn range (& self) -> Range < Self :: ValueType > { self . 0 . range () } fn axis_pixel_range (& self , limit : (i32 , i32)) -> Range < i32 > { let left = self . map (& self . 1 . start , limit) ; let right = self . map (& self . 1 . end , limit) ; left . min (right) .. left . max (right) } }
};
}
