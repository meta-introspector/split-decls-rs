// Generated macro for fixed_from_ethiopian (function)
macro_rules! Depcrate_ethiopianfixed_from_ethiopian {
() => {
// Module: crate::ethiopian
// Provides: {"fixed_from_ethiopian"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L2017>"] pub fn fixed_from_ethiopian (year : i32 , month : u8 , day : u8) -> RataDie { crate :: coptic :: fixed_from_coptic (year , month , day) - ETHIOPIC_TO_COPTIC_OFFSET }
};
}
