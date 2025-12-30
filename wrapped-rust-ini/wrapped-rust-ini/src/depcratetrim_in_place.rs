// Generated macro for trim_in_place (function)
macro_rules! Depcratetrim_in_place {
() => {
// Module: crate
// Provides: {"trim_in_place"}
// Dependencies: {}
fn trim_in_place (string : & mut String) { string . truncate (string . trim_end () . len ()) ; string . drain (.. (string . len () - string . trim_start () . len ())) ; }
};
}
