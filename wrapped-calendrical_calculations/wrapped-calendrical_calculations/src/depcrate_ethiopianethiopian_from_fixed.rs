// Generated macro for ethiopian_from_fixed (function)
macro_rules! Depcrate_ethiopianethiopian_from_fixed {
() => {
// Module: crate::ethiopian
// Provides: {"ethiopian_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L2028>"] pub fn ethiopian_from_fixed (date : RataDie) -> Result < (i32 , u8 , u8) , I32CastError > { crate :: coptic :: coptic_from_fixed (date + ETHIOPIC_TO_COPTIC_OFFSET) }
};
}
