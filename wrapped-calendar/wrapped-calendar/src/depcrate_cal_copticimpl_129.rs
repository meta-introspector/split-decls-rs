// Generated macro for impl_129 (impl)
macro_rules! Depcrate_cal_copticimpl_129 {
() => {
// Module: crate::cal::coptic
// Provides: {"impl_129"}
// Dependencies: {}
impl Date < Coptic > { # [doc = " Construct new Coptic Date."] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " let date_coptic = Date::try_new_coptic(1686, 5, 6)"] # [doc = "     .expect(\"Failed to initialize Coptic Date instance.\");"] # [doc = ""] # [doc = " assert_eq!(date_coptic.era_year().year, 1686);"] # [doc = " assert_eq!(date_coptic.month().ordinal, 5);"] # [doc = " assert_eq!(date_coptic.day_of_month().0, 6);"] # [doc = " ```"] pub fn try_new_coptic (year : i32 , month : u8 , day : u8) -> Result < Date < Coptic > , RangeError > { ArithmeticDate :: try_from_ymd (year , month , day) . map (CopticDateInner) . map (| inner | Date :: from_raw (inner , Coptic)) } }
};
}
