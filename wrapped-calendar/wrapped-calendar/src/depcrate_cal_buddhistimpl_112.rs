// Generated macro for impl_112 (impl)
macro_rules! Depcrate_cal_buddhistimpl_112 {
() => {
// Module: crate::cal::buddhist
// Provides: {"impl_112"}
// Dependencies: {}
impl Date < Buddhist > { # [doc = " Construct a new Buddhist Date."] # [doc = ""] # [doc = " Years are specified as BE years."] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " let date_buddhist = Date::try_new_buddhist(1970, 1, 2)"] # [doc = "     .expect(\"Failed to initialize Buddhist Date instance.\");"] # [doc = ""] # [doc = " assert_eq!(date_buddhist.era_year().year, 1970);"] # [doc = " assert_eq!(date_buddhist.month().ordinal, 1);"] # [doc = " assert_eq!(date_buddhist.day_of_month().0, 2);"] # [doc = " ```"] pub fn try_new_buddhist (year : i32 , month : u8 , day : u8) -> Result < Date < Buddhist > , RangeError > { ArithmeticDate :: new_gregorian :: < BuddhistEra > (year , month , day) . map (BuddhistDateInner) . map (| i | Date :: from_raw (i , Buddhist)) } }
};
}
