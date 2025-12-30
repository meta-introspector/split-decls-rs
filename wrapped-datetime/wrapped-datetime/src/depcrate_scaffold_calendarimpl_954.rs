// Generated macro for impl_954 (impl)
macro_rules! Depcrate_scaffold_calendarimpl_954 {
() => {
// Module: crate::scaffold::calendar
// Provides: {"impl_954"}
// Dependencies: {}
impl FormattableAnyCalendarKind { pub (crate) fn try_from_any_calendar_kind (kind : AnyCalendarKind) -> Option < Self > { use AnyCalendarKind :: * ; let res = match kind { Buddhist => Self :: Buddhist , Chinese => Self :: Chinese , Coptic => Self :: Coptic , Dangi => Self :: Dangi , Ethiopian => Self :: Ethiopian , EthiopianAmeteAlem => Self :: EthiopianAmeteAlem , Gregorian => Self :: Gregorian , Hebrew => Self :: Hebrew , Indian => Self :: Indian , HijriTabularTypeIIFriday => Self :: HijriTabularTypeIIFriday , HijriSimulatedMecca => return None , HijriTabularTypeIIThursday => Self :: HijriTabularTypeIIThursday , HijriUmmAlQura => Self :: HijriUmmAlQura , Iso => return None , Japanese => Self :: Japanese , JapaneseExtended => return None , Persian => Self :: Persian , Roc => Self :: Roc , _ => { debug_assert ! (false , "cross-crate exhaustive match") ; return None ; } } ; Some (res) } pub (crate) fn from_preferences (mut prefs : DateTimeFormatterPreferences) -> Self { if let Some (res) = Self :: try_from_any_calendar_kind (AnyCalendarKind :: new ((& prefs) . into ())) { return res ; } prefs . calendar_algorithm = None ; if let Some (res) = Self :: try_from_any_calendar_kind (AnyCalendarKind :: new ((& prefs) . into ())) { return res ; } debug_assert ! (false , "all locale-default calendars are supported") ; FormattableAnyCalendarKind :: Coptic } }
};
}
