// Generated macro for impl_243 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_243 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_243"}
// Dependencies: {}
impl < Z : TimeZone > TimeValue for Date < Z > { type DateType = Date < Z > ; fn date_floor (& self) -> Date < Z > { self . clone () } fn date_ceil (& self) -> Date < Z > { self . clone () } fn earliest_after_date (date : Date < Z >) -> Self { date } fn subtract (& self , other : & Date < Z >) -> Duration { self . clone () - other . clone () } fn add (& self , other : & Duration) -> Date < Z > { self . clone () + * other } fn ymd (& self , year : i32 , month : u32 , date : u32) -> Self :: DateType { self . timezone () . ymd (year , month , date) } fn from_date (date : Self :: DateType) -> Self { date } }
};
}
