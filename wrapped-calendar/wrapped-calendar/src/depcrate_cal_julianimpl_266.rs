// Generated macro for impl_266 (impl)
macro_rules! Depcrate_cal_julianimpl_266 {
() => {
// Module: crate::cal::julian
// Provides: {"impl_266"}
// Dependencies: {}
impl Date < Julian > { # [doc = " Construct new Julian Date."] # [doc = ""] # [doc = " Years are arithmetic, meaning there is a year 0. Zero and negative years are in BC, with year 0 = 1 BC"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " let date_julian = Date::try_new_julian(1969, 12, 20)"] # [doc = "     .expect(\"Failed to initialize Julian Date instance.\");"] # [doc = ""] # [doc = " assert_eq!(date_julian.era_year().year, 1969);"] # [doc = " assert_eq!(date_julian.month().ordinal, 12);"] # [doc = " assert_eq!(date_julian.day_of_month().0, 20);"] # [doc = " ```"] pub fn try_new_julian (year : i32 , month : u8 , day : u8) -> Result < Date < Julian > , RangeError > { ArithmeticDate :: try_from_ymd (year , month , day) . map (JulianDateInner) . map (| inner | Date :: from_raw (inner , Julian)) } }
};
}
