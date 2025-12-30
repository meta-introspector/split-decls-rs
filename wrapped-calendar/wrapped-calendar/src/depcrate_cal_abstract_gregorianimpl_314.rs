// Generated macro for impl_314 (impl)
macro_rules! Depcrate_cal_abstract_gregorianimpl_314 {
() => {
// Module: crate::cal::abstract_gregorian
// Provides: {"impl_314"}
// Dependencies: {}
impl < Y : GregorianYears > DateFieldsResolver for AbstractGregorian < Y > { type YearInfo = i32 ; fn days_in_provided_month (year : i32 , month : u8) -> u8 { if month == 2 { 28 + calendrical_calculations :: gregorian :: is_leap_year (year) as u8 } else { 30 | month ^ (month >> 3) } } # [inline] fn year_info_from_era (& self , era : & [u8] , era_year : i32 ,) -> Result < Self :: YearInfo , UnknownEraError > { Ok (self . 0 . extended_from_era_year (Some (era) , era_year) ? + Y :: EXTENDED_YEAR_OFFSET) } # [inline] fn year_info_from_extended (& self , extended_year : i32) -> Self :: YearInfo { extended_year + Y :: EXTENDED_YEAR_OFFSET } # [inline] fn reference_year_from_month_day (& self , _month : types :: Month , _day : u8 ,) -> Result < Self :: YearInfo , EcmaReferenceYearError > { Ok (REFERENCE_YEAR) } }
};
}
