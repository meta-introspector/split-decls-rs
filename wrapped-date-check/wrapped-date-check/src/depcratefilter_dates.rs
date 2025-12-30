// Generated macro for filter_dates (function)
macro_rules! Depcratefilter_dates {
() => {
// Module: crate
// Provides: {"filter_dates"}
// Dependencies: {}
fn filter_dates (current_month : Date , min_months_since : u32 , dates_by_file : impl Iterator < Item = (PathBuf , Vec < (usize , Date) >) > ,) -> impl Iterator < Item = (PathBuf , Vec < (usize , Date) >) > { dates_by_file . map (move | (path , dates) | { (path , dates . into_iter () . filter (| (_ , date) | { current_month . months_since (* date) . expect ("found date that is after current month") >= min_months_since }) . collect :: < Vec < _ > > () ,) }) . filter (| (_ , dates) | ! dates . is_empty ()) }
};
}
