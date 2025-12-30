// Generated macro for impl_971 (impl)
macro_rules! Depcrate_scaffold_calendarimpl_971 {
() => {
// Module: crate::scaffold::calendar
// Provides: {"impl_971"}
// Dependencies: {}
impl < O : TimeZoneModel > ConvertCalendar for TimeZoneInfo < O > { type Converted < 'a > = TimeZoneInfo < O > ; # [inline] fn to_calendar < 'a > (& self , _ : & 'a AnyCalendar) -> Self :: Converted < 'a > { * self } }
};
}
