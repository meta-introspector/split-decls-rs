// Generated macro for Date (struct)
macro_rules! Depcrate_dateDate {
() => {
// Module: crate::date
// Provides: {"Date"}
// Dependencies: {}
# [doc = " A date for a given calendar."] # [doc = ""] # [doc = " **The primary definition of this type is in the [`icu_calendar`](https://docs.rs/icu_calendar) crate. Other ICU4X crates re-export it for convenience.**"] # [doc = ""] # [doc = " This can work with wrappers around [`Calendar`] types,"] # [doc = " e.g. `Rc<C>`, via the [`AsCalendar`] trait."] # [doc = ""] # [doc = " This can be constructed  constructed"] # [doc = " from its fields via [`Self::try_new_from_codes()`], or can be constructed with one of the"] # [doc = " `new_<calendar>_date()` per-calendar methods (and then freely converted between calendars)."] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " // Example: creation of ISO date from integers."] # [doc = " let date_iso = Date::try_new_iso(1970, 1, 2)"] # [doc = "     .expect(\"Failed to initialize ISO Date instance.\");"] # [doc = ""] # [doc = " assert_eq!(date_iso.era_year().year, 1970);"] # [doc = " assert_eq!(date_iso.month().ordinal, 1);"] # [doc = " assert_eq!(date_iso.day_of_month().0, 2);"] # [doc = " ```"] pub struct Date < A : AsCalendar > { pub (crate) inner : < A :: Calendar as Calendar > :: DateInner , pub (crate) calendar : A , }
};
}
