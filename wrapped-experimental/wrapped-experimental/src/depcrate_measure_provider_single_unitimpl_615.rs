// Generated macro for impl_615 (impl)
macro_rules! Depcrate_measure_provider_single_unitimpl_615 {
() => {
// Module: crate::measure::provider::single_unit
// Provides: {"impl_615"}
// Dependencies: {}
impl SingleUnit { # [doc = " Appends the short representation of the single unit to the given string."] # [doc = ""] # [doc = " The format of the short representation is as follows:"] # [doc = " 1. If the power is not 1, the power is prefixed with \"P\" followed by the power value."] # [doc = " 2. If the si prefix power is not 0, the si prefix is represented by its base character ('D' for Decimal, 'B' for Binary) followed by the prefix power value."] # [doc = " 3. The unit ID is prefixed with \"I\" and appended to the string."] pub (crate) fn append_short_representation (& self , buff : & mut String) { if self . power != 1 { buff . push ('P') ; let _infallible = write ! (buff , "{}" , self . power) ; } if self . si_prefix . power != 0 { self . si_prefix . append_short_representation (buff) ; } buff . push ('I') ; let _infallible = write ! (buff , "{}" , self . unit_id) ; } }
};
}
