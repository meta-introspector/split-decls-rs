// Generated macro for impl_261 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_261 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_261"}
// Dependencies: {}
impl < T : TimeValue + Clone > DiscreteRanged for Yearly < T > where Range < T > : AsRangedCoord < Value = T > , { fn size (& self) -> usize { let year_start = self . 0 . start . date_ceil () . year () ; let year_end = self . 0 . end . date_floor () . year () ; ((year_end - year_start) . max (- 1) + 1) as usize } fn index_of (& self , value : & T) -> Option < usize > { let year_start = self . 0 . start . date_ceil () . year () ; let year_value = value . date_floor () . year () ; let ret = year_value - year_start ; if ret < 0 { return None ; } Some (ret as usize) } fn from_index (& self , index : usize) -> Option < T > { let year = self . 0 . start . date_ceil () . year () + index as i32 ; let ret = T :: earliest_after_date (self . 0 . start . ymd (year , 1 , 1)) ; if ret . date_ceil () <= self . 0 . start . date_floor () { return Some (self . 0 . start . clone ()) ; } Some (ret) } }
};
}
