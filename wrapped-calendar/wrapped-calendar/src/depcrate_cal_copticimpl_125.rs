// Generated macro for impl_125 (impl)
macro_rules! Depcrate_cal_copticimpl_125 {
() => {
// Module: crate::cal::coptic
// Provides: {"impl_125"}
// Dependencies: {}
impl DateFieldsResolver for Coptic { type YearInfo = i32 ; fn days_in_provided_month (year : i32 , month : u8) -> u8 { if month == 13 { 5 + calendrical_calculations :: coptic :: is_leap_year (year) as u8 } else { 30 } } fn months_in_provided_year (_ : i32) -> u8 { 13 } # [inline] fn year_info_from_era (& self , era : & [u8] , era_year : i32 ,) -> Result < Self :: YearInfo , UnknownEraError > { match era { b"am" => Ok (era_year) , _ => Err (UnknownEraError) , } } # [inline] fn year_info_from_extended (& self , extended_year : i32) -> Self :: YearInfo { extended_year } # [inline] fn reference_year_from_month_day (& self , month : types :: Month , day : u8 ,) -> Result < Self :: YearInfo , EcmaReferenceYearError > { Coptic :: reference_year_from_month_day (month , day) } }
};
}
