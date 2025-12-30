// Generated macro for fmt_array (function)
macro_rules! Depcrate_exportfmt_array {
() => {
// Module: crate::export
// Provides: {"fmt_array"}
// Dependencies: {}
pub fn fmt_array < T : Format > (a : & [T]) { istr (& T :: _format_tag ()) ; for value in a { value . _format_data () ; } }
};
}
