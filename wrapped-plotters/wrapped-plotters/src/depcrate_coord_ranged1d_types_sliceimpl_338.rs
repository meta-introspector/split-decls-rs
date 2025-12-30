// Generated macro for impl_338 (impl)
macro_rules! Depcrate_coord_ranged1d_types_sliceimpl_338 {
() => {
// Module: crate::coord::ranged1d::types::slice
// Provides: {"impl_338"}
// Dependencies: {}
impl < 'a , T : PartialEq > Ranged for RangedSlice < 'a , T > { type FormatOption = DefaultFormatting ; type ValueType = & 'a T ; fn range (& self) -> Range < & 'a T > { & self . 0 [0] .. & self . 0 [self . 0 . len () - 1] } fn map (& self , value : & Self :: ValueType , limit : (i32 , i32)) -> i32 { match self . 0 . iter () . position (| x | & x == value) { Some (pos) => { let pixel_span = limit . 1 - limit . 0 ; let value_span = self . 0 . len () - 1 ; (f64 :: from (limit . 0) + f64 :: from (pixel_span) * (f64 :: from (pos as u32) / f64 :: from (value_span as u32))) . round () as i32 } None => limit . 0 , } } fn key_points < Hint : KeyPointHint > (& self , hint : Hint) -> Vec < Self :: ValueType > { let max_points = hint . max_num_points () ; let mut ret = vec ! [] ; let intervals = (self . 0 . len () - 1) as f64 ; let step = (intervals / max_points as f64 + 1.0) as usize ; for idx in (0 .. self . 0 . len ()) . step_by (step) { ret . push (& self . 0 [idx]) ; } ret } }
};
}
