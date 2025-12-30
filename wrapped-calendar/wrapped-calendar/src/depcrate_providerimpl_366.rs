// Generated macro for impl_366 (impl)
macro_rules! Depcrate_providerimpl_366 {
() => {
// Module: crate::provider
// Provides: {"impl_366"}
// Dependencies: {}
impl WeekdaySet { # [doc = " Creates a new [WeekdaySet] using the provided days."] pub const fn new (days : & [Weekday]) -> Self { let mut i = 0 ; let mut w = 0 ; # [expect (clippy :: indexing_slicing)] while i < days . len () { w |= days [i] . bit_value () ; i += 1 ; } Self (w) } }
};
}
