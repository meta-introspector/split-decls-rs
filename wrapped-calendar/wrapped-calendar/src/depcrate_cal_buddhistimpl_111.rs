// Generated macro for impl_111 (impl)
macro_rules! Depcrate_cal_buddhistimpl_111 {
() => {
// Module: crate::cal::buddhist
// Provides: {"impl_111"}
// Dependencies: {}
impl GregorianYears for BuddhistEra { const EXTENDED_YEAR_OFFSET : i32 = - 543 ; fn extended_from_era_year (& self , era : Option < & [u8] > , year : i32 ,) -> Result < i32 , UnknownEraError > { match era { Some (b"be") | None => Ok (year) , _ => Err (UnknownEraError) , } } fn era_year_from_extended (& self , extended_year : i32 , _month : u8 , _day : u8) -> types :: EraYear { types :: EraYear { era : tinystr ! (16 , "be") , era_index : Some (0) , year : extended_year , extended_year , ambiguity : types :: YearAmbiguity :: CenturyRequired , } } fn debug_name (& self) -> & 'static str { "Buddhist" } fn calendar_algorithm (& self) -> Option < CalendarAlgorithm > { Some (CalendarAlgorithm :: Buddhist) } }
};
}
