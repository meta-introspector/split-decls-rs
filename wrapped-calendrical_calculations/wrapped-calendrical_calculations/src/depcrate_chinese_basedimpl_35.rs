// Generated macro for impl_35 (impl)
macro_rules! Depcrate_chinese_basedimpl_35 {
() => {
// Module: crate::chinese_based
// Provides: {"impl_35"}
// Dependencies: {}
impl ChineseBased for Chinese { fn utc_offset (fixed : RataDie) -> f64 { use crate :: gregorian :: fixed_from_gregorian as gregorian ; if fixed < const { gregorian (1929 , 1 , 1) } { 1397.0 / 180.0 / 24.0 } else { 8.0 / 24.0 } } # [doc = " The equivalent first day in the Chinese calendar (based on inception of the calendar), Feb. 15, -2636"] const EPOCH : RataDie = crate :: gregorian :: fixed_from_gregorian (- 2636 , 2 , 15) ; const DEBUG_NAME : & 'static str = "chinese" ; }
};
}
