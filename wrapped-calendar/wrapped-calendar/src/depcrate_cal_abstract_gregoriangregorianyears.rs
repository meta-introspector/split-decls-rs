// Generated macro for GregorianYears (trait)
macro_rules! Depcrate_cal_abstract_gregorianGregorianYears {
() => {
// Module: crate::cal::abstract_gregorian
// Provides: {"GregorianYears"}
// Dependencies: {}
pub (crate) trait GregorianYears : Clone + core :: fmt :: Debug { const EXTENDED_YEAR_OFFSET : i32 = 0 ; fn extended_from_era_year (& self , era : Option < & [u8] > , year : i32) -> Result < i32 , UnknownEraError > ; fn era_year_from_extended (& self , extended_year : i32 , month : u8 , day : u8) -> EraYear ; fn calendar_algorithm (& self) -> Option < CalendarAlgorithm > { None } fn debug_name (& self) -> & 'static str ; }
};
}
