// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl < T : Bounded > Bounded for OrderedFloat < T > { # [inline] fn min_value () -> Self { OrderedFloat (T :: min_value ()) } # [inline] fn max_value () -> Self { OrderedFloat (T :: max_value ()) } }
};
}
