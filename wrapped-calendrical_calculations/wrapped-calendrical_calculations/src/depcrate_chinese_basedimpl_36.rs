// Generated macro for impl_36 (impl)
macro_rules! Depcrate_chinese_basedimpl_36 {
() => {
// Module: crate::chinese_based
// Provides: {"impl_36"}
// Dependencies: {}
impl ChineseBased for Dangi { fn utc_offset (fixed : RataDie) -> f64 { use crate :: gregorian :: fixed_from_gregorian as gregorian ; if fixed < const { gregorian (1908 , 4 , 1) } { 3809.0 / 450.0 / 24.0 } else if fixed < const { gregorian (1912 , 1 , 1) } { 8.5 / 24.0 } else if fixed < const { gregorian (1954 , 3 , 21) } { 9.0 / 24.0 } else if fixed < const { gregorian (1961 , 8 , 10) } { 8.5 / 24.0 } else { 9.0 / 24.0 } } # [doc = " The first day in the Korean Dangi calendar (based on the founding of Gojoseon), lunar new year -2332"] const EPOCH : RataDie = crate :: gregorian :: fixed_from_gregorian (- 2332 , 2 , 15) ; const DEBUG_NAME : & 'static str = "dangi" ; }
};
}
