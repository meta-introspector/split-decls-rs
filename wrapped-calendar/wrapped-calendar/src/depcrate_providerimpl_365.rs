// Generated macro for impl_365 (impl)
macro_rules! Depcrate_providerimpl_365 {
() => {
// Module: crate::provider
// Provides: {"impl_365"}
// Dependencies: {}
impl WeekdaySet { # [doc = " Returns whether the set contains the day."] pub const fn contains (& self , day : Weekday) -> bool { self . 0 & day . bit_value () != 0 } }
};
}
