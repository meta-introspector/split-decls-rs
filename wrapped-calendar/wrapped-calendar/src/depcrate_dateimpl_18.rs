// Generated macro for impl_18 (impl)
macro_rules! Depcrate_dateimpl_18 {
() => {
// Module: crate::date
// Provides: {"impl_18"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] impl < C : AsCalendar > AsCalendar for Rc < C > { type Calendar = C :: Calendar ; # [inline] fn as_calendar (& self) -> & Self :: Calendar { self . as_ref () . as_calendar () } }
};
}
