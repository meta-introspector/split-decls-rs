// Generated macro for impl_394 (impl)
macro_rules! Depcrate_calendarimpl_394 {
() => {
// Module: crate::calendar
// Provides: {"impl_394"}
// Dependencies: {}
impl CalendarEventStore { # [doc = " Construct a store that has the current date styled."] # [doc = ""] # [doc = " `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or"] # [doc = " your own type that implements [`Into<Style>`])."] # [doc = ""] # [doc = " [`Color`]: ratatui_core::style::Color"] # [cfg (feature = "std")] pub fn today < S : Into < Style > > (style : S) -> Self { use time :: OffsetDateTime ; let mut res = Self :: default () ; res . add (OffsetDateTime :: now_local () . unwrap_or_else (| _ | OffsetDateTime :: now_utc ()) . date () , style . into () ,) ; res } # [doc = " Add a date and style to the store"] # [doc = ""] # [doc = " `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or"] # [doc = " your own type that implements [`Into<Style>`])."] # [doc = ""] # [doc = " [`Color`]: ratatui_core::style::Color"] pub fn add < S : Into < Style > > (& mut self , date : Date , style : S) { let _ = self . 0 . insert (date , style . into ()) ; } # [doc = " Helper for trait impls"] fn lookup_style (& self , date : Date) -> Style { self . 0 . get (& date) . copied () . unwrap_or_default () } }
};
}
