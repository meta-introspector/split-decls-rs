// Generated macro for impl_367 (impl)
macro_rules! Depcrate_providerimpl_367 {
() => {
// Module: crate::provider
// Provides: {"impl_367"}
// Dependencies: {}
impl Weekday { # [doc = " Defines the bit order used for encoding and reading weekend days."] const fn bit_value (self) -> u8 { match self { Weekday :: Monday => 1 << 6 , Weekday :: Tuesday => 1 << 5 , Weekday :: Wednesday => 1 << 4 , Weekday :: Thursday => 1 << 3 , Weekday :: Friday => 1 << 2 , Weekday :: Saturday => 1 << 1 , Weekday :: Sunday => 1 << 0 , } } }
};
}
