// Generated macro for impl_244 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_244 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_244"}
// Dependencies: {}
impl < Z : TimeZone > TimeValue for DateTime < Z > { type DateType = Date < Z > ; fn date_floor (& self) -> Date < Z > { self . date () } fn date_ceil (& self) -> Date < Z > { if self . time () . num_seconds_from_midnight () > 0 { self . date () + Duration :: days (1) } else { self . date () } } fn earliest_after_date (date : Date < Z >) -> DateTime < Z > { date . and_hms (0 , 0 , 0) } fn subtract (& self , other : & DateTime < Z >) -> Duration { self . clone () - other . clone () } fn add (& self , other : & Duration) -> DateTime < Z > { self . clone () + * other } fn ymd (& self , year : i32 , month : u32 , date : u32) -> Self :: DateType { self . timezone () . ymd (year , month , date) } fn from_date (date : Self :: DateType) -> Self { date . and_hms (0 , 0 , 0) } }
};
}
