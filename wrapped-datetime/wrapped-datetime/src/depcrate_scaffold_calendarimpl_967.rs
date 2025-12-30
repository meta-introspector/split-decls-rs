// Generated macro for impl_967 (impl)
macro_rules! Depcrate_scaffold_calendarimpl_967 {
() => {
// Module: crate::scaffold::calendar
// Provides: {"impl_967"}
// Dependencies: {}
impl < C : IntoAnyCalendar , A : AsCalendar < Calendar = C > > ConvertCalendar for Date < A > { type Converted < 'a > = Date < Ref < 'a , AnyCalendar > > ; # [inline] fn to_calendar < 'a > (& self , calendar : & 'a AnyCalendar) -> Self :: Converted < 'a > { self . to_calendar (Ref (calendar)) } }
};
}
