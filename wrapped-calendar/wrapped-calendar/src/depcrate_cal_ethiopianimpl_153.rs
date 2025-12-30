// Generated macro for impl_153 (impl)
macro_rules! Depcrate_cal_ethiopianimpl_153 {
() => {
// Module: crate::cal::ethiopian
// Provides: {"impl_153"}
// Dependencies: {}
impl Date < Ethiopian > { # [doc = " Construct new Ethiopian Date."] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::calendar::cal::EthiopianEraStyle;"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " let date_ethiopian ="] # [doc = "     Date::try_new_ethiopian(EthiopianEraStyle::AmeteMihret, 2014, 8, 25)"] # [doc = "         .expect(\"Failed to initialize Ethopic Date instance.\");"] # [doc = ""] # [doc = " assert_eq!(date_ethiopian.era_year().year, 2014);"] # [doc = " assert_eq!(date_ethiopian.month().ordinal, 8);"] # [doc = " assert_eq!(date_ethiopian.day_of_month().0, 25);"] # [doc = " ```"] pub fn try_new_ethiopian (era_style : EthiopianEraStyle , year : i32 , month : u8 , day : u8 ,) -> Result < Date < Ethiopian > , RangeError > { let year = Ethiopian (era_style) . year_info_from_extended (year) ; ArithmeticDate :: try_from_ymd (year , month , day) . map (CopticDateInner) . map (EthiopianDateInner) . map (| inner | Date :: from_raw (inner , Ethiopian (era_style))) } }
};
}
