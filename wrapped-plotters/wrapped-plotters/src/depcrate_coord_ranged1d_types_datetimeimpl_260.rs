// Generated macro for impl_260 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_260 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_260"}
// Dependencies: {}
impl < T : TimeValue + Clone > Ranged for Yearly < T > where Range < T > : AsRangedCoord < Value = T > , { type FormatOption = NoDefaultFormatting ; type ValueType = T ; fn range (& self) -> Range < T > { self . 0 . start . clone () .. self . 0 . end . clone () } fn map (& self , value : & Self :: ValueType , limit : (i32 , i32)) -> i32 { T :: map_coord (value , & self . 0 . start , & self . 0 . end , limit) } fn key_points < HintType : KeyPointHint > (& self , hint : HintType) -> Vec < Self :: ValueType > { if hint . weight () . allow_light_points () && self . size () <= hint . bold_points () * 2 { return Monthly (self . 0 . clone ()) . key_points (hint) ; } let max_points = hint . max_num_points () ; let start_date = self . 0 . start . date_ceil () ; let end_date = self . 0 . end . date_floor () ; let mut start_year = start_date . year () ; let mut start_month = start_date . month () ; let start_day = start_date . day () ; let end_year = end_date . year () ; let end_month = end_date . month () ; if start_day != 1 { start_month += 1 ; if start_month == 13 { start_month = 1 ; start_year += 1 ; } } generate_yearly_keypoints (max_points , start_year , start_month , end_year , end_month , & self . 0 . start ,) } }
};
}
