// Generated macro for impl_223 (impl)
macro_rules! Depcrate_cal_isoimpl_223 {
() => {
// Module: crate::cal::iso
// Provides: {"impl_223"}
// Dependencies: {}
impl Date < Iso > { # [doc = " Construct a new ISO date from integers."] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " let date_iso = Date::try_new_iso(1970, 1, 2)"] # [doc = "     .expect(\"Failed to initialize ISO Date instance.\");"] # [doc = ""] # [doc = " assert_eq!(date_iso.era_year().year, 1970);"] # [doc = " assert_eq!(date_iso.month().ordinal, 1);"] # [doc = " assert_eq!(date_iso.day_of_month().0, 2);"] # [doc = " ```"] pub fn try_new_iso (year : i32 , month : u8 , day : u8) -> Result < Date < Iso > , RangeError > { ArithmeticDate :: new_gregorian :: < IsoEra > (year , month , day) . map (IsoDateInner) . map (| i | Date :: from_raw (i , Iso)) } }
};
}
