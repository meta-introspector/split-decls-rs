// Generated macro for impl_256 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_256 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_256"}
// Dependencies: {}
impl < T : TimeValue + Clone > DiscreteRanged for Monthly < T > where Range < T > : AsRangedCoord < Value = T > , { fn size (& self) -> usize { let (start_year , start_month) = { let ceil = self . 0 . start . date_ceil () ; (ceil . year () , ceil . month ()) } ; let (end_year , end_month) = { let floor = self . 0 . end . date_floor () ; (floor . year () , floor . month ()) } ; ((end_year - start_year) . max (0) * 12 + (1 - start_month as i32) + (end_month as i32 - 1) + 1) . max (0) as usize } fn index_of (& self , value : & T) -> Option < usize > { let this_year = value . date_floor () . year () ; let this_month = value . date_floor () . month () ; let start_year = self . 0 . start . date_ceil () . year () ; let start_month = self . 0 . start . date_ceil () . month () ; let ret = (this_year - start_year) . max (0) * 12 + (1 - start_month as i32) + (this_month as i32 - 1) ; if ret >= 0 { return Some (ret as usize) ; } None } fn from_index (& self , index : usize) -> Option < T > { if index == 0 { return Some (T :: earliest_after_date (self . 0 . start . date_ceil ())) ; } let index_from_start_year = index + (self . 0 . start . date_ceil () . month () - 1) as usize ; let year = self . 0 . start . date_ceil () . year () + index_from_start_year as i32 / 12 ; let month = index_from_start_year % 12 ; Some (T :: earliest_after_date (self . 0 . start . ymd (year , month as u32 + 1 , 1 ,))) } }
};
}
