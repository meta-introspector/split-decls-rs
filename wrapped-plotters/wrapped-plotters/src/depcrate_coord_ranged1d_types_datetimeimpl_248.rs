// Generated macro for impl_248 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_248 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_248"}
// Dependencies: {}
impl < D > Ranged for RangedDate < D > where D : Datelike + TimeValue + Sub < D , Output = Duration > + Add < Duration , Output = D > + Clone , { type FormatOption = DefaultFormatting ; type ValueType = D ; fn range (& self) -> Range < D > { self . 0 . clone () .. self . 1 . clone () } fn map (& self , value : & Self :: ValueType , limit : (i32 , i32)) -> i32 { TimeValue :: map_coord (value , & self . 0 , & self . 1 , limit) } fn key_points < HintType : KeyPointHint > (& self , hint : HintType) -> Vec < Self :: ValueType > { let max_points = hint . max_num_points () ; let mut ret = vec ! [] ; let total_days = (self . 1 . clone () - self . 0 . clone ()) . num_days () ; let total_weeks = (self . 1 . clone () - self . 0 . clone ()) . num_weeks () ; if total_days > 0 && total_days as usize <= max_points { for day_idx in 0 ..= total_days { ret . push (self . 0 . clone () + Duration :: days (day_idx)) ; } return ret ; } if total_weeks > 0 && total_weeks as usize <= max_points { for day_idx in 0 ..= total_weeks { ret . push (self . 0 . clone () + Duration :: weeks (day_idx)) ; } return ret ; } if total_weeks == 0 { ret . push (self . 0 . clone ()) ; return ret ; } let week_per_point = ((total_weeks as f64) / (max_points as f64)) . ceil () as usize ; for idx in 0 ..= (total_weeks as usize / week_per_point) { ret . push (self . 0 . clone () + Duration :: weeks ((idx * week_per_point) as i64)) ; } ret } }
};
}
