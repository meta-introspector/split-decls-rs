// Generated macro for impl_207 (impl)
macro_rules! Depcrate_cal_indianimpl_207 {
() => {
// Module: crate::cal::indian
// Provides: {"impl_207"}
// Dependencies: {}
impl DateFieldsResolver for Indian { type YearInfo = i32 ; fn days_in_provided_month (year : i32 , month : u8) -> u8 { 30 + (month <= 6) as u8 - (month == 1 && ! calendrical_calculations :: gregorian :: is_leap_year (year + YEAR_OFFSET)) as u8 } # [inline] fn year_info_from_era (& self , era : & [u8] , era_year : i32 ,) -> Result < Self :: YearInfo , UnknownEraError > { match era { b"shaka" => Ok (era_year) , _ => Err (UnknownEraError) , } } # [inline] fn year_info_from_extended (& self , extended_year : i32) -> Self :: YearInfo { extended_year } # [inline] fn reference_year_from_month_day (& self , month : types :: Month , day : u8 ,) -> Result < Self :: YearInfo , EcmaReferenceYearError > { let (ordinal_month , false) = (month . number () , month . is_leap ()) else { return Err (EcmaReferenceYearError :: MonthCodeNotInCalendar) ; } ; let shaka_year = if ordinal_month < 10 || (ordinal_month == 10 && day <= 10) { 1894 } else { 1893 } ; Ok (shaka_year) } }
};
}
