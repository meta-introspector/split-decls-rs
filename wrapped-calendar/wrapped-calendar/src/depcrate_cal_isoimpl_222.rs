// Generated macro for impl_222 (impl)
macro_rules! Depcrate_cal_isoimpl_222 {
() => {
// Module: crate::cal::iso
// Provides: {"impl_222"}
// Dependencies: {}
impl GregorianYears for IsoEra { fn extended_from_era_year (& self , era : Option < & [u8] > , year : i32 ,) -> Result < i32 , UnknownEraError > { match era { Some (b"default") | None => Ok (year) , Some (_) => Err (UnknownEraError) , } } fn era_year_from_extended (& self , extended_year : i32 , _month : u8 , _day : u8) -> types :: EraYear { types :: EraYear { era_index : Some (0) , era : tinystr ! (16 , "default") , year : extended_year , extended_year , ambiguity : types :: YearAmbiguity :: Unambiguous , } } fn debug_name (& self) -> & 'static str { "ISO" } }
};
}
