// Generated macro for impl_242 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_242 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_242"}
// Dependencies: {}
impl TimeValue for NaiveDate { type DateType = NaiveDate ; fn date_floor (& self) -> NaiveDate { * self } fn date_ceil (& self) -> NaiveDate { * self } fn earliest_after_date (date : NaiveDate) -> Self { date } fn subtract (& self , other : & NaiveDate) -> Duration { * self - * other } fn add (& self , other : & Duration) -> NaiveDate { * self + * other } fn ymd (& self , year : i32 , month : u32 , date : u32) -> Self :: DateType { NaiveDate :: from_ymd (year , month , date) } fn from_date (date : Self :: DateType) -> Self { date } }
};
}
