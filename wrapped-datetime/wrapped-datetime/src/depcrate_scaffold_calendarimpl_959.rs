// Generated macro for impl_959 (impl)
macro_rules! Depcrate_scaffold_calendarimpl_959 {
() => {
// Module: crate::scaffold::calendar
// Provides: {"impl_959"}
// Dependencies: {}
# [doc = " A version of [`FormattableAnyCalendar`] that is smaller on the stack."] impl UntaggedFormattableAnyCalendar { pub (crate) fn into_tagged (self) -> FormattableAnyCalendar { let kind = FormattableAnyCalendarKind :: try_from_any_calendar_kind (self . any_calendar . kind ()) . unwrap_or_else (| | { debug_assert ! (false , "unreachable by invariant") ; FormattableAnyCalendarKind :: Coptic }) ; FormattableAnyCalendar { any_calendar : self . any_calendar , kind , } } pub (crate) fn any_calendar (& self) -> & AnyCalendar { & self . any_calendar } pub (crate) fn take_any_calendar (self) -> AnyCalendar { self . any_calendar } }
};
}
