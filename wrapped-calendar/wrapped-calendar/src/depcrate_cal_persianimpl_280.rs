// Generated macro for impl_280 (impl)
macro_rules! Depcrate_cal_persianimpl_280 {
() => {
// Module: crate::cal::persian
// Provides: {"impl_280"}
// Dependencies: {}
impl DateFieldsResolver for Persian { type YearInfo = i32 ; fn days_in_provided_month (year : i32 , month : u8) -> u8 { 30 + (month <= 6) as u8 - (month == 12 && ! calendrical_calculations :: persian :: is_leap_year (year)) as u8 } # [inline] fn year_info_from_era (& self , era : & [u8] , era_year : i32 ,) -> Result < Self :: YearInfo , UnknownEraError > { match era { b"ap" => Ok (era_year) , _ => Err (UnknownEraError) , } } # [inline] fn year_info_from_extended (& self , extended_year : i32) -> Self :: YearInfo { extended_year } # [inline] fn reference_year_from_month_day (& self , month : types :: Month , day : u8 ,) -> Result < Self :: YearInfo , EcmaReferenceYearError > { let (ordinal_month , false) = (month . number () , month . is_leap ()) else { return Err (EcmaReferenceYearError :: MonthCodeNotInCalendar) ; } ; let persian_year = if ordinal_month < 10 || (ordinal_month == 10 && day <= 10) { 1351 } else { 1350 } ; Ok (persian_year) } }
};
}
