// Generated macro for impl_211 (impl)
macro_rules! Depcrate_cal_indianimpl_211 {
() => {
// Module: crate::cal::indian
// Provides: {"impl_211"}
// Dependencies: {}
impl Date < Indian > { # [doc = " Construct new Indian Date, with year provided in the Śaka era."] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " let date_indian = Date::try_new_indian(1891, 10, 12)"] # [doc = "     .expect(\"Failed to initialize Indian Date instance.\");"] # [doc = ""] # [doc = " assert_eq!(date_indian.era_year().year, 1891);"] # [doc = " assert_eq!(date_indian.month().ordinal, 10);"] # [doc = " assert_eq!(date_indian.day_of_month().0, 12);"] # [doc = " ```"] pub fn try_new_indian (year : i32 , month : u8 , day : u8) -> Result < Date < Indian > , RangeError > { ArithmeticDate :: try_from_ymd (year , month , day) . map (IndianDateInner) . map (| inner | Date :: from_raw (inner , Indian)) } }
};
}
