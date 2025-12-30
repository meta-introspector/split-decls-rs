// Generated macro for collect_dates (function)
macro_rules! Depcratecollect_dates {
() => {
// Module: crate
// Provides: {"collect_dates"}
// Dependencies: {}
fn collect_dates (paths : impl Iterator < Item = PathBuf >) -> BTreeMap < PathBuf , Vec < (usize , Date) > > { let date_regex = make_date_regex () ; let mut data = BTreeMap :: new () ; for path in paths { let text = fs :: read_to_string (& path) . unwrap () ; let dates = collect_dates_from_file (& date_regex , & text) ; if ! dates . is_empty () { data . insert (path , dates) ; } } data }
};
}
