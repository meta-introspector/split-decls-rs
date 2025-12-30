// Generated macro for impl_164 (impl)
macro_rules! Depcrate_cal_gregorianimpl_164 {
() => {
// Module: crate::cal::gregorian
// Provides: {"impl_164"}
// Dependencies: {}
impl GregorianYears for CeBce { fn extended_from_era_year (& self , era : Option < & [u8] > , year : i32 ,) -> Result < i32 , UnknownEraError > { match era { None => Ok (year) , Some (b"ad" | b"ce") => Ok (year) , Some (b"bce" | b"bc") => Ok (1 - year) , Some (_) => Err (UnknownEraError) , } } fn era_year_from_extended (& self , extended_year : i32 , _month : u8 , _day : u8) -> types :: EraYear { if extended_year > 0 { types :: EraYear { era : tinystr ! (16 , "ce") , era_index : Some (1) , year : extended_year , extended_year , ambiguity : match extended_year { ..= 999 => types :: YearAmbiguity :: EraAndCenturyRequired , 1000 ..= 1949 => types :: YearAmbiguity :: CenturyRequired , 1950 ..= 2049 => types :: YearAmbiguity :: Unambiguous , 2050 .. => types :: YearAmbiguity :: CenturyRequired , } , } } else { types :: EraYear { era : tinystr ! (16 , "bce") , era_index : Some (0) , year : 1 - extended_year , extended_year , ambiguity : types :: YearAmbiguity :: EraAndCenturyRequired , } } } fn debug_name (& self) -> & 'static str { "Gregorian" } fn calendar_algorithm (& self) -> Option < CalendarAlgorithm > { Some (CalendarAlgorithm :: Gregory) } }
};
}
