// Generated macro for impl_970 (impl)
macro_rules! Depcrate_scaffold_calendarimpl_970 {
() => {
// Module: crate::scaffold::calendar
// Provides: {"impl_970"}
// Dependencies: {}
impl < C : IntoAnyCalendar , A : AsCalendar < Calendar = C > , Z : Copy > ConvertCalendar for ZonedDateTime < A , Z > { type Converted < 'a > = ZonedDateTime < Ref < 'a , AnyCalendar > , Z > ; # [inline] fn to_calendar < 'a > (& self , calendar : & 'a AnyCalendar) -> Self :: Converted < 'a > { ZonedDateTime { date : self . date . to_calendar (Ref (calendar)) , time : self . time , zone : self . zone , } } }
};
}
