// Generated macro for impl_265 (impl)
macro_rules! Depcrate_cal_julianimpl_265 {
() => {
// Module: crate::cal::julian
// Provides: {"impl_265"}
// Dependencies: {}
impl Julian { # [doc = " Construct a new Julian Calendar"] pub fn new () -> Self { Self } # [doc = " Returns the date of (Orthodox) Easter in the given year."] pub fn easter (year : i32) -> Date < Self > { Date :: from_rata_die (calendrical_calculations :: julian :: easter (year) , Self) } }
};
}
