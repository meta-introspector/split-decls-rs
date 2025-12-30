// Generated macro for get_args (function)
macro_rules! Depcrateget_args {
() => {
// Module: crate
// Provides: {"get_args"}
// Dependencies: {}
fn get_args (full_event : & analyzeme :: Event < '_ >) -> Option < FxHashMap < String , String > > { if ! full_event . additional_data . is_empty () { Some (full_event . additional_data . iter () . enumerate () . map (| (i , arg) | (format ! ("arg{}" , i) . to_string () , arg . to_string ())) . collect () ,) } else { None } }
};
}
