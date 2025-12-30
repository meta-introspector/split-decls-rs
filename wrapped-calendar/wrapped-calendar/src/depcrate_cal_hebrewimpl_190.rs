// Generated macro for impl_190 (impl)
macro_rules! Depcrate_cal_hebrewimpl_190 {
() => {
// Module: crate::cal::hebrew
// Provides: {"impl_190"}
// Dependencies: {}
impl Date < Hebrew > { # [doc = " This method uses an ordinal month, which is probably not what you want."] # [doc = ""] # [doc = " Use [`Date::try_new_from_codes`]"] # [deprecated (since = "2.1.0" , note = "use `Date::try_new_from_codes`")] pub fn try_new_hebrew (year : i32 , ordinal_month : u8 , day : u8 ,) -> Result < Date < Hebrew > , RangeError > { let year = HebrewYear :: compute (year) ; ArithmeticDate :: try_from_ymd (year , ordinal_month , day) . map (HebrewDateInner) . map (| inner | Date :: from_raw (inner , Hebrew)) } }
};
}
