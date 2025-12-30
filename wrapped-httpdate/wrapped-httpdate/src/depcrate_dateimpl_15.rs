// Generated macro for impl_15 (impl)
macro_rules! Depcrate_dateimpl_15 {
() => {
// Module: crate::date
// Provides: {"impl_15"}
// Dependencies: {}
impl From < HttpDate > for SystemTime { fn from (v : HttpDate) -> SystemTime { let leap_years = ((v . year - 1) - 1968) / 4 - ((v . year - 1) - 1900) / 100 + ((v . year - 1) - 1600) / 400 ; let mut ydays = match v . mon { 1 => 0 , 2 => 31 , 3 => 59 , 4 => 90 , 5 => 120 , 6 => 151 , 7 => 181 , 8 => 212 , 9 => 243 , 10 => 273 , 11 => 304 , 12 => 334 , _ => unreachable ! () , } + v . day as u64 - 1 ; if is_leap_year (v . year) && v . mon > 2 { ydays += 1 ; } let days = (v . year as u64 - 1970) * 365 + leap_years as u64 + ydays ; UNIX_EPOCH + Duration :: from_secs (v . sec as u64 + v . min as u64 * 60 + v . hour as u64 * 3600 + days * 86400 ,) } }
};
}
