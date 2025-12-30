// Generated macro for coptic_from_fixed (function)
macro_rules! Depcrate_copticcoptic_from_fixed {
() => {
// Module: crate::coptic
// Provides: {"coptic_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1990>"] pub fn coptic_from_fixed (date : RataDie) -> Result < (i32 , u8 , u8) , I32CastError > { let year = (4 * (date - COPTIC_EPOCH) + 1463) . div_euclid (1461) ; let year = i64_to_i32 (year) ? ; let month = ((date - fixed_from_coptic (year , 1 , 1)) . div_euclid (30) + 1) as u8 ; let day = (date + 1 - fixed_from_coptic (year , month , 1)) as u8 ; Ok ((year , month , day)) }
};
}
