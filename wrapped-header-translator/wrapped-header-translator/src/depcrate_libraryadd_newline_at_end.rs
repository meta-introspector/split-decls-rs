// Generated macro for add_newline_at_end (function)
macro_rules! Depcrate_libraryadd_newline_at_end {
() => {
// Module: crate::library
// Provides: {"add_newline_at_end"}
// Dependencies: {}
fn add_newline_at_end (item : & mut Item) { item . as_table_mut () . unwrap () . iter_mut () . last () . unwrap () . 1 . as_value_mut () . unwrap () . decor_mut () . set_suffix ("\n") ; }
};
}
