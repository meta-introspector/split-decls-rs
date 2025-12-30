// Generated macro for impl_166 (impl)
macro_rules! Depcrate_cal_gregorianimpl_166 {
() => {
// Module: crate::cal::gregorian
// Provides: {"impl_166"}
// Dependencies: {}
impl Date < Gregorian > { # [doc = " Construct a new Gregorian Date."] # [doc = ""] # [doc = " Years are specified as ISO years."] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " // Conversion from ISO to Gregorian"] # [doc = " let date_gregorian = Date::try_new_gregorian(1970, 1, 2)"] # [doc = "     .expect(\"Failed to initialize Gregorian Date instance.\");"] # [doc = ""] # [doc = " assert_eq!(date_gregorian.era_year().year, 1970);"] # [doc = " assert_eq!(date_gregorian.month().ordinal, 1);"] # [doc = " assert_eq!(date_gregorian.day_of_month().0, 2);"] # [doc = " ```"] pub fn try_new_gregorian (year : i32 , month : u8 , day : u8) -> Result < Date < Gregorian > , RangeError > { ArithmeticDate :: new_gregorian :: < CeBce > (year , month , day) . map (GregorianDateInner) . map (| i | Date :: from_raw (i , Gregorian)) } }
};
}
