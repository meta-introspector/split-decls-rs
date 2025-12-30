// Generated macro for impl_284 (impl)
macro_rules! Depcrate_cal_persianimpl_284 {
() => {
// Module: crate::cal::persian
// Provides: {"impl_284"}
// Dependencies: {}
impl Date < Persian > { # [doc = " Construct new Persian Date."] # [doc = ""] # [doc = " Has no negative years, only era is the AH/AP."] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " let date_persian = Date::try_new_persian(1392, 4, 25)"] # [doc = "     .expect(\"Failed to initialize Persian Date instance.\");"] # [doc = ""] # [doc = " assert_eq!(date_persian.era_year().year, 1392);"] # [doc = " assert_eq!(date_persian.month().ordinal, 4);"] # [doc = " assert_eq!(date_persian.day_of_month().0, 25);"] # [doc = " ```"] pub fn try_new_persian (year : i32 , month : u8 , day : u8) -> Result < Date < Persian > , RangeError > { ArithmeticDate :: try_from_ymd (year , month , day) . map (PersianDateInner) . map (| inner | Date :: from_raw (inner , Persian)) } }
};
}
