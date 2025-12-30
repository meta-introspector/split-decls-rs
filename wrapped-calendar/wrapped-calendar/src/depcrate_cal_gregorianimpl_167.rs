// Generated macro for impl_167 (impl)
macro_rules! Depcrate_cal_gregorianimpl_167 {
() => {
// Module: crate::cal::gregorian
// Provides: {"impl_167"}
// Dependencies: {}
impl Gregorian { # [doc = " Returns the date of Easter in the given year."] pub fn easter (year : i32) -> Date < Self > { Date :: from_rata_die (calendrical_calculations :: gregorian :: easter (year) , Self) } }
};
}
