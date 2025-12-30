// Generated macro for impl_149 (impl)
macro_rules! Depcrate_cal_ethiopianimpl_149 {
() => {
// Module: crate::cal::ethiopian
// Provides: {"impl_149"}
// Dependencies: {}
impl DateFieldsResolver for Ethiopian { type YearInfo = i32 ; fn days_in_provided_month (year : Self :: YearInfo , month : u8) -> u8 { Coptic :: days_in_provided_month (year , month) } fn months_in_provided_year (year : Self :: YearInfo) -> u8 { Coptic :: months_in_provided_year (year) } # [inline] fn year_info_from_era (& self , era : & [u8] , era_year : i32 ,) -> Result < Self :: YearInfo , UnknownEraError > { match (self . era_style () , era) { (EthiopianEraStyle :: AmeteMihret , b"am") => Ok (era_year + AMETE_MIHRET_OFFSET) , (_ , b"aa") => Ok (era_year + AMETE_ALEM_OFFSET) , (_ , _) => Err (UnknownEraError) , } } # [inline] fn year_info_from_extended (& self , extended_year : i32) -> Self :: YearInfo { extended_year + if self . 0 == EthiopianEraStyle :: AmeteMihret { AMETE_MIHRET_OFFSET } else { AMETE_ALEM_OFFSET } } # [inline] fn reference_year_from_month_day (& self , month : types :: Month , day : u8 ,) -> Result < Self :: YearInfo , EcmaReferenceYearError > { crate :: cal :: Coptic :: reference_year_from_month_day (month , day) } }
};
}
