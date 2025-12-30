// Generated macro for impl_296 (impl)
macro_rules! Depcrate_cal_rocimpl_296 {
() => {
// Module: crate::cal::roc
// Provides: {"impl_296"}
// Dependencies: {}
impl GregorianYears for RocEra { const EXTENDED_YEAR_OFFSET : i32 = 1911 ; fn extended_from_era_year (& self , era : Option < & [u8] > , year : i32 ,) -> Result < i32 , UnknownEraError > { match era { None => Ok (year) , Some (b"roc") => Ok (year) , Some (b"broc") => Ok (1 - year) , Some (_) => Err (UnknownEraError) , } } fn era_year_from_extended (& self , extended_year : i32 , _month : u8 , _day : u8) -> types :: EraYear { if extended_year > 0 { types :: EraYear { era : tinystr ! (16 , "roc") , era_index : Some (1) , year : extended_year , extended_year , ambiguity : types :: YearAmbiguity :: CenturyRequired , } } else { types :: EraYear { era : tinystr ! (16 , "broc") , era_index : Some (0) , year : 1 - extended_year , extended_year , ambiguity : types :: YearAmbiguity :: EraAndCenturyRequired , } } } fn debug_name (& self) -> & 'static str { "ROC" } fn calendar_algorithm (& self) -> Option < CalendarAlgorithm > { Some (CalendarAlgorithm :: Roc) } }
};
}
