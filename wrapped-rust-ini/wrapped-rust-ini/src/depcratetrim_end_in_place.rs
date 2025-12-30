// Generated macro for trim_end_in_place (function)
macro_rules! Depcratetrim_end_in_place {
() => {
// Module: crate
// Provides: {"trim_end_in_place"}
// Dependencies: {}
fn trim_end_in_place (string : & mut String) { string . truncate (string . trim_end () . len ()) ; }
};
}
