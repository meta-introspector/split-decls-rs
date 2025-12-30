// Generated macro for get_year_name_from_map (function)
macro_rules! Depcrate_provider_neoget_year_name_from_map {
() => {
// Module: crate::provider::neo
// Provides: {"get_year_name_from_map"}
// Dependencies: {}
pub (crate) fn get_year_name_from_map < 'a > (map : & 'a YearNamesMap < '_ > , year : & PotentialUtf8 ,) -> Option < & 'a str > { let idx = map . a () . binary_search_by (| x | x . cmp (year)) . ok () ? ; map . b () . get (idx) }
};
}
