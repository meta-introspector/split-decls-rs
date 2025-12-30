// Generated macro for impl_388 (impl)
macro_rules! Depcrate_typesimpl_388 {
() => {
// Module: crate::types
// Provides: {"impl_388"}
// Dependencies: {}
impl YearInfo { # [doc = " Get *some* year number that can be displayed"] # [doc = ""] # [doc = " Gets the era year for era calendars, and the related ISO year for cyclic calendars."] pub fn era_year_or_related_iso (self) -> i32 { match self { YearInfo :: Era (e) => e . year , YearInfo :: Cyclic (c) => c . related_iso , } } # [doc = " Get the extended year (See [`Date::extended_year`](crate::Date::extended_year))"] # [doc = " for more information"] pub fn extended_year (self) -> i32 { match self { YearInfo :: Era (e) => e . extended_year , YearInfo :: Cyclic (c) => c . related_iso , } } # [doc = " Get the era year information, if available"] pub fn era (self) -> Option < EraYear > { match self { Self :: Era (e) => Some (e) , Self :: Cyclic (_) => None , } } # [doc = " Get the cyclic year informat, if available"] pub fn cyclic (self) -> Option < CyclicYear > { match self { Self :: Era (_) => None , Self :: Cyclic (c) => Some (c) , } } }
};
}
