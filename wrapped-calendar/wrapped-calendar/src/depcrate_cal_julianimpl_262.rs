// Generated macro for impl_262 (impl)
macro_rules! Depcrate_cal_julianimpl_262 {
() => {
// Module: crate::cal::julian
// Provides: {"impl_262"}
// Dependencies: {}
impl DateFieldsResolver for Julian { type YearInfo = i32 ; fn days_in_provided_month (year : i32 , month : u8) -> u8 { if month == 2 { 28 + calendrical_calculations :: julian :: is_leap_year (year) as u8 } else { 30 | month ^ (month >> 3) } } # [inline] fn year_info_from_era (& self , era : & [u8] , era_year : i32 ,) -> Result < Self :: YearInfo , UnknownEraError > { match era { b"ad" | b"ce" => Ok (era_year) , b"bc" | b"bce" => Ok (1 - era_year) , _ => Err (UnknownEraError) , } } # [inline] fn year_info_from_extended (& self , extended_year : i32) -> Self :: YearInfo { extended_year } # [inline] fn reference_year_from_month_day (& self , month : types :: Month , day : u8 ,) -> Result < Self :: YearInfo , EcmaReferenceYearError > { let (ordinal_month , false) = (month . number () , month . is_leap ()) else { return Err (EcmaReferenceYearError :: MonthCodeNotInCalendar) ; } ; let julian_year = if ordinal_month < 12 || (ordinal_month == 12 && day <= 18) { 1972 } else { 1971 } ; Ok (julian_year) } }
};
}
