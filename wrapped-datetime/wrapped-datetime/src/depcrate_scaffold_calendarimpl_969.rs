// Generated macro for impl_969 (impl)
macro_rules! Depcrate_scaffold_calendarimpl_969 {
() => {
// Module: crate::scaffold::calendar
// Provides: {"impl_969"}
// Dependencies: {}
impl < C : IntoAnyCalendar , A : AsCalendar < Calendar = C > > ConvertCalendar for DateTime < A > { type Converted < 'a > = DateTime < Ref < 'a , AnyCalendar > > ; # [inline] fn to_calendar < 'a > (& self , calendar : & 'a AnyCalendar) -> Self :: Converted < 'a > { DateTime { date : self . date . to_calendar (Ref (calendar)) , time : self . time , } } }
};
}
