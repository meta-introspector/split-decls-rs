// Generated macro for impl_245 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_245 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_245"}
// Dependencies: {}
impl TimeValue for NaiveDateTime { type DateType = NaiveDate ; fn date_floor (& self) -> NaiveDate { self . date () } fn date_ceil (& self) -> NaiveDate { if self . time () . num_seconds_from_midnight () > 0 { self . date () + Duration :: days (1) } else { self . date () } } fn earliest_after_date (date : NaiveDate) -> NaiveDateTime { date . and_hms (0 , 0 , 0) } fn subtract (& self , other : & NaiveDateTime) -> Duration { * self - * other } fn add (& self , other : & Duration) -> NaiveDateTime { * self + * other } fn ymd (& self , year : i32 , month : u32 , date : u32) -> Self :: DateType { NaiveDate :: from_ymd (year , month , date) } fn from_date (date : Self :: DateType) -> Self { date . and_hms (0 , 0 , 0) } }
};
}
