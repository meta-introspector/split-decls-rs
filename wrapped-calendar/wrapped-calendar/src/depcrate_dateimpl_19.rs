// Generated macro for impl_19 (impl)
macro_rules! Depcrate_dateimpl_19 {
() => {
// Module: crate::date
// Provides: {"impl_19"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] impl < C : AsCalendar > AsCalendar for Arc < C > { type Calendar = C :: Calendar ; # [inline] fn as_calendar (& self) -> & Self :: Calendar { self . as_ref () . as_calendar () } }
};
}
